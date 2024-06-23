//! Actor framework for interacting with a connected pool of players.
//!
//! # Entry points
//! * [`Actor::start`] - Initialize the actor.
//! * [`Handle`] - Communication handle.
//! * [`Message`] - Message spec.

use std::{collections::HashMap, convert::Infallible, time::Duration};

use axum::{
    async_trait,
    extract::{
        ws::{self, WebSocket},
        FromRequestParts,
    },
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    time::sleep,
};
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

    #[instrument(skip_all, level = Level::TRACE)]
    async fn on_tick(map: &mut HashMap<PlayerId, Player>) {
        // PERF(low): Have a fixed buffer higher up in the call stack.
        let mut made_games = Vec::new();
        for player in map.values_mut() {
            if let Some(ref invite) = player.inbound_invite {
                if !player.notified_of_invite {
                    let notice = serde_json::to_string(&invite).unwrap();
                    player.socket.send(ws::Message::Text(notice)).await.unwrap();
                    player.notified_of_invite = true;
                } else if player.accepted_invite {
                    // Deref to drop the borrow.
                    made_games.push((player.id(), *invite));
                }
            }
        }

        for (p1id, p2id) in made_games {
            if let Some(p1) = map.remove(&p1id) {
                if let Some(p2) = map.remove(&p2id) {
                    tokio::spawn(async move {
                        let ids = [p1.id(), p2.id()];
                        let result = play_game(p1, p2).await;
                        info!(game.members = ?ids, game.result = ?result);
                    });
                }
            }
        }
    }

    #[instrument(skip_all)]
    async fn run_loop(mut receiver: mpsc::Receiver<Message>) -> Result<()> {
        let mut pool = HashMap::<PlayerId, Player>::new();
        loop {
            select! {
                Some(message) = receiver.recv() => {
                    Self::on_message(&mut pool, message).await.ok();
                },
                _ = sleep(Duration::from_secs(2)) => {
                    Self::on_tick(&mut pool).await
                },
                // TODO: await a handle from the server for shutdown.
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
            Message::Invite(invite) => {
                let Invite {
                    sender_id,
                    target_id,
                } = invite;

                let log = move |msg| {
                    debug!(
                        invite.sender = sender_id.to_string(),
                        invite.target = target_id.to_string(),
                        invite.message = msg,
                        invite.ignore = true,
                    )
                };

                let Some(target) = map.get_mut(&target_id) else {
                    log("target was not found");
                    return Ok(());
                };
                target.inbound_invite = Some(sender_id);

                let Some(sender) = map.get_mut(&sender_id) else {
                    log("sender was not found");
                    return Ok(());
                };
                sender.outbound_invite = Some(target_id);

                map.get_mut(&target_id)
                    .and_then(|target| target.inbound_invite.take());
            }
        };
        Ok(())
    }
}

#[allow(unused)]
async fn play_game(player1: Player, player2: Player) -> Result<()> {
    Ok(()) // TODO: Proxy the connections
}

#[derive(Debug, Serialize)]
pub struct Player {
    id: PlayerId,
    /// Has this player invited another player to a game?
    outbound_invite: Option<PlayerId>,
    inbound_invite: Option<PlayerId>,
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
            outbound_invite: Default::default(),
            inbound_invite: Default::default(),
            notified_of_invite: Default::default(),
        })
    }

    /// Returns the [`PlayerId`] for this [`Player`].
    pub fn id(&self) -> PlayerId {
        self.id
    }
}

/// Unique identifier for the [`Player`].
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId {
    uuid: Uuid,
}

impl Default for PlayerId {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
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

/// Pair of [`PlayerId`] with a target and sender requesting a game.
pub struct Invite {
    pub sender_id: PlayerId,
    pub target_id: PlayerId,
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
    Invite(Invite),
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
