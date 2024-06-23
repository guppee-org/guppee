//! Actor framework for interacting with a connected pool of players.
//!
//! # Entry points
//! * [`Actor::start`] - Initialize the actor.
//! * [`Handle`] - Communication handle.
//! * [`Message`] - Message spec.

use std::{collections::VecDeque, convert::Infallible};

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
use tracing::{instrument, Level};
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

    #[instrument(skip_all)]
    async fn run_loop(mut receiver: mpsc::Receiver<Message>) -> Result<()> {
        let mut queue = VecDeque::<Player>::new();
        while let Some(message) = receiver.recv().await {
            match message {
                Message::Register(socket) => {
                    queue.push_back(Player::new(socket).await?);
                }
                Message::PlayerList(reply) => {
                    let player_list = queue.iter().map(Player::id).collect();
                    reply.send(player_list).unwrap();
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Player {
    id: PlayerId,
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
        Ok(Self { socket, id })
    }

    /// Returns the [`PlayerId`] for this [`Player`].
    pub fn id(&self) -> PlayerId {
        self.id
    }
}

/// Unique identifier for the [`Player`].
#[derive(Clone, Copy, Deserialize, Serialize, Debug, Default)]
pub struct PlayerId {
    uuid: Uuid,
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
