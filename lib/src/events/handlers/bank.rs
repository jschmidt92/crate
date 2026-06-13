use crate::{events::DomainEventHandler, models::DomainEvent, services::bank, shared::EventError};

#[derive(Debug, Clone, Copy, Default)]
pub struct BankActorCreatedHandler;

impl DomainEventHandler for BankActorCreatedHandler {
    fn name(&self) -> &'static str {
        "bank.actor_created"
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        match event {
            DomainEvent::ActorCreated(actor_created) => {
                bank::create_actor_account(&actor_created.actor, &actor_created.starting).map_err(
                    |error| EventError::HandlerFailed {
                        handler: self.name(),
                        event: event.name(),
                        message: error.to_string(),
                    },
                )?;

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actor, ActorCreated, ActorSnapshot, ActorStartingConfig};

    #[test]
    fn actor_created_creates_bank_account_for_actor_uid() {
        let handler = BankActorCreatedHandler;
        let actor = Actor::from_snapshot(ActorSnapshot::new("76561198000000000", "Tester"));
        let event =
            DomainEvent::ActorCreated(ActorCreated::new(actor, ActorStartingConfig::default()));

        assert!(handler.handle(&event).is_ok());
    }
}
