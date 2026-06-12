use crate::{
    models::{Actor, ActorCreated, ActorSnapshot, DomainEvent},
    repositories::ActorRepository,
    shared::ActorError,
};

#[derive(Debug, Clone)]
pub struct ActorInitResult {
    pub actor: Actor,
    pub created: bool,
    pub events: Vec<DomainEvent>,
}

pub struct ActorService<R> {
    repository: R,
}

impl<R> ActorService<R>
where
    R: ActorRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn init_or_create(&self, snapshot: ActorSnapshot) -> Result<ActorInitResult, ActorError> {
        validate_snapshot(&snapshot)?;

        if let Some(mut actor) = self.repository.find_by_uid(&snapshot.uid)? {
            actor.apply_snapshot(snapshot);
            let actor = self.repository.save(actor)?;

            return Ok(ActorInitResult {
                actor,
                created: false,
                events: Vec::new(),
            });
        }

        let actor = self.repository.save(Actor::from_snapshot(snapshot))?;
        let event = DomainEvent::ActorCreated(ActorCreated::new(actor.clone()));

        Ok(ActorInitResult {
            actor,
            created: true,
            events: vec![event],
        })
    }

    pub fn get(&self, uid: &str) -> Result<Option<Actor>, ActorError> {
        validate_uid(uid)?;
        self.repository.find_by_uid(uid)
    }

    pub fn save(&self, actor: Actor) -> Result<Actor, ActorError> {
        validate_uid(&actor.uid)?;
        validate_name(&actor.name)?;
        self.repository.save(actor)
    }

    pub fn delete(&self, uid: &str) -> Result<(), ActorError> {
        validate_uid(uid)?;
        self.repository.delete(uid)
    }
}

fn validate_snapshot(snapshot: &ActorSnapshot) -> Result<(), ActorError> {
    validate_uid(&snapshot.uid)?;
    validate_name(&snapshot.name)?;
    Ok(())
}

fn validate_uid(uid: &str) -> Result<(), ActorError> {
    if uid.trim().is_empty() {
        return Err(ActorError::InvalidUid);
    }

    Ok(())
}

fn validate_name(name: &str) -> Result<(), ActorError> {
    if name.trim().is_empty() {
        return Err(ActorError::InvalidName);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{ActorLifeState, ActorRank, ActorStance},
        repositories::InMemoryActorRepository,
    };

    #[test]
    fn init_or_create_creates_missing_actor_and_emits_event() {
        let service = ActorService::new(InMemoryActorRepository::new());
        let result = service
            .init_or_create(ActorSnapshot::new("76561198000000000", "Tester"))
            .expect("actor should be created");

        assert!(result.created);
        assert_eq!(result.actor.uid, "76561198000000000");
        assert_eq!(result.events.len(), 1);
        assert!(matches!(result.events[0], DomainEvent::ActorCreated(_)));
    }

    #[test]
    fn init_or_create_updates_existing_actor_without_event() {
        let service = ActorService::new(InMemoryActorRepository::new());
        service
            .init_or_create(ActorSnapshot::new("76561198000000000", "Tester"))
            .expect("actor should be created");

        let mut snapshot = ActorSnapshot::new("76561198000000000", "Renamed");
        snapshot.position = [1.0, 2.0, 3.0];
        snapshot.stance = ActorStance::Crouch;
        snapshot.rank = ActorRank::Sergeant;
        snapshot.life_state = ActorLifeState::Injured;

        let result = service
            .init_or_create(snapshot)
            .expect("actor should be updated");

        assert!(!result.created);
        assert_eq!(result.actor.name, "Renamed");
        assert_eq!(result.actor.position, [1.0, 2.0, 3.0]);
        assert!(result.events.is_empty());
    }

    #[test]
    fn init_or_create_rejects_invalid_snapshot() {
        let service = ActorService::new(InMemoryActorRepository::new());

        let error = service
            .init_or_create(ActorSnapshot::new("", "Tester"))
            .expect_err("empty uid should fail");

        assert_eq!(error, ActorError::InvalidUid);
    }
}
