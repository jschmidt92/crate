use serde::{Deserialize, Serialize};

use super::Actor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomainEvent {
    ActorCreated(ActorCreated),
}

impl DomainEvent {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ActorCreated(_) => "actor.created",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorCreated {
    pub actor: Actor,
}

impl ActorCreated {
    pub const fn new(actor: Actor) -> Self {
        Self { actor }
    }
}
