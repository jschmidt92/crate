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
