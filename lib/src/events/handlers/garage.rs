use crate::{
    events::DomainEventHandler, models::DomainEvent, repositories::InMemoryGarageRepository,
    services::GarageService, shared::EventError,
};

pub struct GarageActorCreatedHandler {
    service: GarageService<InMemoryGarageRepository>,
}

impl GarageActorCreatedHandler {
    pub fn new(service: GarageService<InMemoryGarageRepository>) -> Self {
        Self { service }
    }
}

impl Default for GarageActorCreatedHandler {
    fn default() -> Self {
        Self::new(GarageService::new(InMemoryGarageRepository::new()))
    }
}

impl DomainEventHandler for GarageActorCreatedHandler {
    fn name(&self) -> &'static str {
        "garage.actor_created"
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        match event {
            DomainEvent::ActorCreated(actor_created) => self
                .service
                .create_actor_garage(&actor_created.actor.uid)
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
