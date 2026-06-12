use crate::{models::Actor, shared::ActorError};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait ActorRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<Actor>, ActorError>;
    fn save(&self, actor: Actor) -> Result<Actor, ActorError>;
    fn delete(&self, uid: &str) -> Result<(), ActorError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryActorRepository {
    actors: Arc<RwLock<HashMap<String, Actor>>>,
}

impl InMemoryActorRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ActorRepository for InMemoryActorRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<Actor>, ActorError> {
        let actors = self
            .actors
            .read()
            .map_err(|error| ActorError::Repository(error.to_string()))?;

        Ok(actors.get(uid).cloned())
    }

    fn save(&self, actor: Actor) -> Result<Actor, ActorError> {
        let mut actors = self
            .actors
            .write()
            .map_err(|error| ActorError::Repository(error.to_string()))?;

        actors.insert(actor.uid.clone(), actor.clone());
        Ok(actor)
    }

    fn delete(&self, uid: &str) -> Result<(), ActorError> {
        let mut actors = self
            .actors
            .write()
            .map_err(|error| ActorError::Repository(error.to_string()))?;

        actors.remove(uid);
        Ok(())
    }
}
