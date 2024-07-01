use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId {
    repr: Uuid,
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self {
            repr: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    id: PlayerId,
    #[serde(default)]
    display_name: Option<String>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: Default::default(),
            display_name: Default::default(),
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}
