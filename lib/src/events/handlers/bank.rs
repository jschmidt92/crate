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
                bank::create_actor_account(&actor_created.actor).map_err(|error| {
                    EventError::HandlerFailed {
                        handler: self.name(),
                        event: event.name(),
                        message: error.to_string(),
                    }
                })?;

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actor, ActorCreated, ActorSnapshot};

    #[test]
    fn actor_created_provisions_bank_profile_for_numeric_uid() {
        let handler = BankActorCreatedHandler;
        let actor = Actor::from_snapshot(ActorSnapshot::new("76561198000000000", "Tester"));
        let event = DomainEvent::ActorCreated(ActorCreated::new(actor));

        assert!(handler.handle(&event).is_ok());
    }

    #[test]
    fn actor_created_reports_invalid_bank_actor_uid() {
        let handler = BankActorCreatedHandler;
        let actor = Actor::from_snapshot(ActorSnapshot::new("not-a-steam-id", "Tester"));
        let event = DomainEvent::ActorCreated(ActorCreated::new(actor));

        assert!(matches!(
            handler.handle(&event),
            Err(EventError::HandlerFailed {
                handler: "bank.actor_created",
                event: "actor.created",
                ..
            })
        ));
    }
}
