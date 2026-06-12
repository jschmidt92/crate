use crate::{
    events::DomainEventHandler, models::DomainEvent, repositories::InMemoryLockerRepository,
    services::LockerService, shared::EventError,
};

pub struct LockerActorCreatedHandler {
    service: LockerService<InMemoryLockerRepository>,
}

impl LockerActorCreatedHandler {
    pub fn new(service: LockerService<InMemoryLockerRepository>) -> Self {
        Self { service }
    }
}

impl Default for LockerActorCreatedHandler {
    fn default() -> Self {
        Self::new(LockerService::new(InMemoryLockerRepository::new()))
    }
}

impl DomainEventHandler for LockerActorCreatedHandler {
    fn name(&self) -> &'static str {
        "locker.actor_created"
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        match event {
            DomainEvent::ActorCreated(actor_created) => self
                .service
                .create_actor_locker(&actor_created.actor.uid)
                .map(|_| ())
                .map_err(|error| EventError::HandlerFailed {
                    handler: self.name(),
                    event: event.name(),
                    message: error.to_string(),
                }),
        }
    }
}
