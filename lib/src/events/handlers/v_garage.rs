use crate::{
    events::DomainEventHandler, models::DomainEvent, repositories::InMemoryVGarageRepository,
    services::VGarageService, shared::EventError,
};

pub struct VGarageActorCreatedHandler {
    service: VGarageService<InMemoryVGarageRepository>,
}

impl VGarageActorCreatedHandler {
    pub fn new(service: VGarageService<InMemoryVGarageRepository>) -> Self {
        Self { service }
    }
}

impl Default for VGarageActorCreatedHandler {
    fn default() -> Self {
        Self::new(VGarageService::new(InMemoryVGarageRepository::new()))
    }
}

impl DomainEventHandler for VGarageActorCreatedHandler {
    fn name(&self) -> &'static str {
        "v_garage.actor_created"
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        match event {
            DomainEvent::ActorCreated(actor_created) => self
                .service
                .create_actor_garage(
                    &actor_created.actor.uid,
                    &actor_created.starting.virtual_garage,
                )
                .map(|_| ())
                .map_err(|error| EventError::HandlerFailed {
                    handler: self.name(),
                    event: event.name(),
                    message: error.to_string(),
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actor, ActorCreated, ActorSnapshot, ActorStartingConfig, DomainEvent};

    #[test]
    fn actor_created_creates_garage_for_actor_uid() {
        let handler = VGarageActorCreatedHandler::default();
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));
        let mut starting = ActorStartingConfig::default();
        starting.virtual_garage.cars = vec!["B_Quadbike_01_F".to_string()];
        let event = DomainEvent::ActorCreated(ActorCreated::new(actor, starting));

        assert!(handler.handle(&event).is_ok());
    }
}
