use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerId {
    repr: Uuid,
}

impl Default for PlayerId {
    fn default() -> Self {
        Self {
            repr: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Player {
    id: PlayerId,
}
