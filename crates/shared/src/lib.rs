pub mod player;
pub mod time;

use serde::{Deserialize, Serialize};

pub use crate::{
    player::{Player, PlayerId},
    time::{Age, Timestamp},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerList {
    sent: Timestamp,
    content: Vec<Player>,
}

impl FromIterator<Player> for PlayerList {
    fn from_iter<T: IntoIterator<Item = Player>>(iter: T) -> Self {
        PlayerList {
            sent: Default::default(),
            content: iter.into_iter().collect(),
        }
    }
}

impl AsRef<Timestamp> for PlayerList {
    fn as_ref(&self) -> &Timestamp {
        &self.sent
    }
}

impl IntoIterator for PlayerList {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Player;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Invited {
    sent: Timestamp,
    sender: Player,
}

impl Invited {
    pub fn new(sender: Player) -> Self {
        Self {
            sent: Default::default(),
            sender,
        }
    }
}

impl AsRef<Timestamp> for Invited {
    fn as_ref(&self) -> &Timestamp {
        &self.sent
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetractInvite {
    sent: Timestamp,
    player_id: PlayerId,
}

impl RetractInvite {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            sent: Default::default(),
            player_id,
        }
    }
}

/// Messages that originate from the server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Server {
    /// Server notifies the client that the member pool has updated.
    // A more performant way to make this update would be to only communicate
    // the addition or removal of players rather than sending down the whole
    // list on each update. This is fine for now though.
    PlayerList(PlayerList),
    /// Server notifies the client that another player has invited them to play.
    Invited(Invited),
    /// Server notifies the client that any invites from the attached player
    /// should be dropped.
    RetractInvite(RetractInvite),
}
