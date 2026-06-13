use crate::{
    events::DomainEventHandler, models::DomainEvent, repositories::InMemoryVLockerRepository,
    services::VLockerService, shared::EventError,
};

pub struct VLockerActorCreatedHandler {
    service: VLockerService<InMemoryVLockerRepository>,
}

impl VLockerActorCreatedHandler {
    pub fn new(service: VLockerService<InMemoryVLockerRepository>) -> Self {
        Self { service }
    }
}

impl Default for VLockerActorCreatedHandler {
    fn default() -> Self {
        Self::new(VLockerService::new(InMemoryVLockerRepository::new()))
    }
}

impl DomainEventHandler for VLockerActorCreatedHandler {
    fn name(&self) -> &'static str {
        "v_locker.actor_created"
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        match event {
            DomainEvent::ActorCreated(actor_created) => self
                .service
                .create_actor_locker(
                    &actor_created.actor.uid,
                    &actor_created.starting.virtual_arsenal,
                )
                .map(|_| ())
                .map_err(|error| EventError::HandlerFailed {
                    handler: self.name(),
                    event: event.name(),
                    message: error.to_string(),
                }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actor, ActorCreated, ActorSnapshot, ActorStartingConfig, DomainEvent};

    #[test]
    fn actor_created_creates_locker_for_actor_uid() {
        let handler = VLockerActorCreatedHandler::default();
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));
        let mut starting = ActorStartingConfig::default();
        starting.virtual_arsenal.weapons = vec!["hgun_P07_F".to_string()];
        let event = DomainEvent::ActorCreated(ActorCreated::new(actor, starting));

        assert!(handler.handle(&event).is_ok());
    }
}
