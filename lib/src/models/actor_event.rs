use serde::{Deserialize, Serialize};

use super::{Actor, ActorStartingConfig};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorCreated {
    pub actor: Actor,
    pub starting: ActorStartingConfig,
}

impl ActorCreated {
    pub const fn new(actor: Actor, starting: ActorStartingConfig) -> Self {
        Self { actor, starting }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorDisconnected {
    pub uid: String,
}

impl ActorDisconnected {
    pub fn new(uid: impl Into<String>) -> Self {
        Self { uid: uid.into() }
    }
}
