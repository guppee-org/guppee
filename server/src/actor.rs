//! Actor framework for interacting with a connected pool of players.
//!
//! # Entry points
//! * [`Actor::start`] - Initialize the actor.
//! * [`Handle`] - Communication handle.
//! * [`Message`] - Message spec.

use std::{collections::HashMap, convert::Infallible};

use axum::{
    async_trait,
    extract::{
        ws::{self, WebSocket},
        FromRequestParts,
    },
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, info, instrument, Level};
use uuid::Uuid;

use crate::Result;

/// Message buffer for the [`Actor`]'s [`mpsc::Receiver`].
const BUFFER_SIZE: usize = 10;

/// Actor framework for connected [`Player`]s.
///
/// Receives [`Message`]s through the [`Handle`].
pub struct Actor;

impl Actor {
    /// Start the [`Actor`] in a top-level task
    #[instrument]
    pub fn start() -> Handle {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);
        tokio::spawn(async move {
            if let Err(err) = Self::run_loop(receiver).await {
                // TODO: Recoverable errors.
                // Just doing this for now to get it done with.
                panic!("FATAL: {err:?}")
            }
        });
        Handle { sender }
    }

    async fn on_tick(map: &mut HashMap<PlayerId, Player>) {
        let mut made_games = Vec::new();
        for player in map.values_mut() {
            if let Some(ref invite) = player.invite_from {
                if !player.notified_of_invite {
                    let notice = serde_json::to_string(&invite).unwrap();
                    player.socket.send(ws::Message::Text(notice)).await.unwrap();
                    player.notified_of_invite = true;
                } else if player.accepted_invite {
                    made_games.push((player.id(), *invite)); // Deref to drop
                                                             // the borrow.
                }
            }
        }

        for (player_1_id, player_2_id) in made_games.drain(..) {
            if let Some(player_1) = map.remove(&player_1_id) {
                if let Some(player_2) = map.remove(&player_2_id) {
                    tokio::spawn(async move {
                        let ids = [player_1.id(), player_2.id()];
                        let result = play_game(player_1, player_2).await;
                        info!(game.members = ?ids, game.result = ?result);
                    });
                }
            }
        }
    }

    #[instrument(skip_all)]
    async fn run_loop(mut receiver: mpsc::Receiver<Message>) -> Result<()> {
        let mut player_map = HashMap::<PlayerId, Player>::new();
        loop {
            let ticker = tokio::time::sleep(std::time::Duration::from_secs(2));
            tokio::select! {
                Some(message) = receiver.recv() => {
                    Self::on_message(&mut player_map, message).await.ok();
                },
                _ = ticker => {
                    Self::on_tick(&mut player_map).await
                },
                _ = tokio::signal::ctrl_c() => {
                    info!("received ctrl C");
                    break;
                },
            }
        }
        Ok(())
    }

    async fn on_message(map: &mut HashMap<PlayerId, Player>, message: Message) -> Result<()> {
        match message {
            Message::Register(socket) => {
                let player = Player::new(socket).await?;
                if let Some(existing) = map.insert(player.id(), player) {
                    debug!("a player attempted to register twice: {existing:?}");
                }
            }
            Message::PlayerList(reply) => {
                let player_list = map.keys().cloned().collect();
                let send_result = reply.send(player_list);
                debug!(?send_result);
            }
            Message::Invite { sender, target } => {
                match map.get_mut(&target) {
                    Some(target_player) => {
                        target_player.invite_from = Some(sender);
                        match map.get_mut(&sender) {
                            Some(target_sender) => {
                                target_sender.sent_invite = Some(target);
                            }
                            None => {
                                debug!(
                                    invite.sender = sender.to_string(),
                                    invite.target = target.to_string(),
                                    "sender was not found, ignoring invite"
                                );
                                let target = map
                                    .get_mut(&target)
                                    .expect("just found it one and no other mutable references");
                                target.invite_from = None;
                            }
                        }
                    }
                    None => {
                        debug!(
                            invite.sender = sender.to_string(),
                            invite.target = target.to_string(),
                            "target was not found, ignoring invite"
                        );
                    }
                };
                match map.get_mut(&sender) {
                    Some(player) => player.sent_invite = Some(target),
                    None => {
                        debug!(
                            invite.sender = sender.to_string(),
                            invite.target = target.to_string(),
                            "sender was not found, ignoring invite"
                        );
                    }
                };
            }
        };
        Ok(())
    }
}

#[allow(unused)]
async fn play_game(player1: Player, player2: Player) -> Result<()> {
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Player {
    id: PlayerId,
    /// Has this player invited another player to a game?
    sent_invite: Option<PlayerId>,
    invite_from: Option<PlayerId>,
    notified_of_invite: bool,
    accepted_invite: bool,
    #[serde(skip_serializing)] // This cannot be serialized to the client
    #[allow(dead_code)]
    socket: WebSocket,
}

impl Player {
    /// Returns a [`Player`] and sends a [`PlayerId`] to the client.
    #[instrument(skip_all)]
    async fn new(mut socket: WebSocket) -> Result<Self> {
        let id = PlayerId::default();
        let initial_response = serde_json::to_string(&id)?;
        socket.send(ws::Message::Text(initial_response)).await?;
        Ok(Self {
            socket,
            id,
            accepted_invite: Default::default(),
            sent_invite: Default::default(),
            invite_from: Default::default(),
            notified_of_invite: Default::default(),
        })
    }

    /// Returns the [`PlayerId`] for this [`Player`].
    pub fn id(&self) -> PlayerId {
        self.id
    }
}

/// Unique identifier for the [`Player`].
#[derive(Clone, Copy, Deserialize, Serialize, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlayerId {
    uuid: Uuid,
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

/// Cheaply cloneable message sender for the [`Actor`] to receive on.
#[derive(Clone, Debug)]
pub struct Handle {
    sender: mpsc::Sender<Message>,
}

impl Handle {
    /// Retrieve a list of [`PlayerId`]s connected to the server.
    #[instrument(skip_all, level = Level::TRACE)]
    pub async fn players(&self) -> Result<Vec<PlayerId>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Message::PlayerList(sender)).await?;
        let player_list = receiver.await?;
        Ok(player_list)
    }

    /// Register an incoming [`WebSocket`] as a [`Player`]
    #[instrument(skip_all, level = Level::TRACE)]
    pub async fn register(&self, socket: WebSocket) -> Result<()> {
        self.sender.send(Message::Register(socket)).await?;
        Ok(())
    }
}

/// Requests and their related responses for the [`Actor`].
pub enum Message {
    /// Register a [`WebSocket`] for conversion to a [`Player`], adding to the
    /// pool.
    // TODO: make sure this works.
    // Once the socket is assigned a [`PlayerId`] a JSON representation of a [`Player`]
    // will be pushed through the socket for client side confirmation.
    Register(WebSocket),
    /// Request a [`Vec`] of [`PlayerId`] from the [`Actor`].
    PlayerList(oneshot::Sender<Vec<PlayerId>>),
    Invite {
        sender: PlayerId,
        target: PlayerId,
    },
}

// Allows the Handle to be easily extracted from a request handler.
#[async_trait]
impl FromRequestParts<crate::App> for Handle {
    type Rejection = Infallible;

    async fn from_request_parts(
        _: &mut Parts,
        state: &crate::App,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.handle.clone())
    }
}
