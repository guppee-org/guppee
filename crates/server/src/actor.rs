#![allow(unused_mut, unused_variables)]

use std::convert::Infallible;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use futures::SinkExt;
use shared::{own, Player, PlayerId, PlayerList, ServerMessage};
use tokio::sync::{mpsc, watch};

use crate::{error::Error, socket::Socket, Result};

const BUFFER_SIZE: usize = 50;

pub struct Pool;

impl Pool {
    #[allow(unused_mut)]
    pub fn start() -> Handle {
        let (sender, mut incoming_socket) = mpsc::channel::<Socket>(BUFFER_SIZE);
        let (player_sender, player_receiver) = watch::channel(PlayerList::new());
        tokio::spawn(async move {
            while let Some(socket) = incoming_socket.recv().await {
                own!(player_sender, player_receiver);
                tokio::spawn(fan_out(player_sender, player_receiver, socket));
            }
        });
        Handle { sender }
    }
}

async fn fan_out(
    mut ps: watch::Sender<PlayerList>,
    mut pr: watch::Receiver<PlayerList>,
    socket: Socket,
) -> Result<()> {
    let mut player = PlayerState::new(socket);
    let player_list = {
        let borrow = pr.borrow_and_update();
        borrow.to_owned()
    };
    let item = ServerMessage::PlayerList(player_list);
    player.socket.send(item).await?;
    Ok(())
}

#[allow(unused)]
#[derive(Debug)]
pub struct PlayerState {
    player: Player,
    outbound_invite: Option<PlayerId>,
    inbound_invite: Option<PlayerId>,
    notified_of_invite: bool,
    accepted_invite: bool,
    socket: Socket,
}

impl PlayerState {
    fn new(mut socket: Socket) -> Self {
        Self {
            socket,
            player: Default::default(),
            accepted_invite: Default::default(),
            outbound_invite: Default::default(),
            inbound_invite: Default::default(),
            notified_of_invite: Default::default(),
        }
    }

    /// Returns the [`PlayerId`].
    pub fn id(&self) -> PlayerId {
        self.player.id()
    }
}

/// Cheaply cloneable message sender for the [`Actor`] to receive on.
#[derive(Debug)]
pub struct Handle {
    sender: mpsc::Sender<Socket>,
}

impl Clone for Handle {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl Handle {
    pub async fn register_socket(&self, socket: Socket) -> Result<()> {
        self.sender.send(socket).await.map_err(Error::erased)?;
        Ok(())
    }
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
