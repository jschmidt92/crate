use crate::{models::DomainEvent, shared::EventError};
use std::sync::Arc;

pub trait DomainEventHandler: Send + Sync {
    fn name(&self) -> &'static str;
    fn handle(&self, event: &DomainEvent) -> Result<(), EventError>;
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: DomainEvent);

    fn publish_all(&self, events: &[DomainEvent]) {
        for event in events {
            self.publish(event.clone());
        }
    }
}

#[derive(Clone, Default)]
pub struct EventBus {
    handlers: Vec<Arc<dyn DomainEventHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<H>(mut self, handler: H) -> Self
    where
        H: DomainEventHandler + 'static,
    {
        self.handlers.push(Arc::new(handler));
        self
    }

    pub fn publish(&self, event: &DomainEvent) -> Vec<EventError> {
        self.handlers
            .iter()
            .filter_map(|handler| handler.handle(event).err())
            .collect()
    }

    pub fn publish_all(&self, events: &[DomainEvent]) -> Vec<EventError> {
        events
            .iter()
            .flat_map(|event| self.publish(event))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actor, ActorCreated, ActorStartingConfig};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct RecordingHandler {
        received_events: Arc<Mutex<Vec<&'static str>>>,
    }

    impl DomainEventHandler for RecordingHandler {
        fn name(&self) -> &'static str {
            "recording"
        }

        fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
            self.received_events
                .lock()
                .expect("received event log should not be poisoned")
                .push(event.name());

            Ok(())
        }
    }

    #[test]
    fn publish_all_dispatches_every_event_to_handlers() {
        let received_events = Arc::new(Mutex::new(Vec::new()));
        let bus = EventBus::new().subscribe(RecordingHandler {
            received_events: received_events.clone(),
        });
        let actor = Actor::from_snapshot(crate::models::ActorSnapshot::new(
            "76561198000000000",
            "Tester",
        ));
        let events = vec![DomainEvent::ActorCreated(ActorCreated::new(
            actor,
            ActorStartingConfig::default(),
        ))];

        let errors = bus.publish_all(&events);

        assert!(errors.is_empty());
        let received_events = received_events
            .lock()
            .expect("received event log should not be poisoned");
        assert_eq!(received_events.as_slice(), ["actor.created"]);
    }
}
