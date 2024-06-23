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

const BUFFER_SIZE: usize = 10;

pub struct Actor;

impl Actor {
    #[instrument(level = Level::TRACE, skip_all)]
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

    #[instrument(level = Level::TRACE, skip_all)]
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

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    #[allow(dead_code)] // Just have not written the code yet.
    socket: WebSocket,
}

impl Player {
    /// Returns a [`Player`] and sends a [`PlayerId`] to the client.
    async fn new(mut socket: WebSocket) -> Result<Self> {
        let id = PlayerId::default();
        let initial_response = serde_json::to_string(&id)?;
        socket.send(ws::Message::Text(initial_response)).await?;
        Ok(Self { socket, id })
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug, Default)]
pub struct PlayerId {
    uuid: Uuid,
}

#[derive(Clone, Debug)]
pub struct Handle {
    sender: mpsc::Sender<Message>,
}

impl Handle {
    /// Retrieve a list of [`PlayerId`]s connected to the server.
    pub async fn players(&self) -> Result<Vec<PlayerId>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Message::PlayerList(sender)).await.unwrap();
        Ok(receiver.await?)
    }

    /// Register an incoming [`WebSocket`] as a [`Player`]
    pub async fn register(&self, socket: WebSocket) {
        self.sender.send(Message::Register(socket)).await.ok();
    }
}

pub enum Message {
    Register(WebSocket),
    PlayerList(oneshot::Sender<Vec<PlayerId>>),
}

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
