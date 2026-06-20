use super::ActorFeature;
use forge_lib::{
    events::EventPublisher,
    models::{Actor, ActorDisconnected, ActorSnapshot, DomainEvent},
    repositories::ActorRepository,
    shared::ActorError,
};

impl<R, E> ActorFeature<R, E>
where
    R: ActorRepository,
    E: EventPublisher,
{
    pub(crate) fn save_snapshot(&self, snapshot: ActorSnapshot) -> Result<Actor, ActorError> {
        self.service.save_snapshot(snapshot)
    }

    pub(crate) fn disconnect(&self, snapshot: ActorSnapshot) -> Result<Actor, ActorError> {
        let actor = self.service.disconnect(snapshot)?;
        self.events
            .publish(DomainEvent::ActorDisconnected(ActorDisconnected::new(
                actor.uid.clone(),
            )));
        Ok(actor)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), ActorError> {
        self.service.delete(uid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_lib::{
        events::EventPublisher, models::DomainEvent, repositories::InMemoryActorRepository,
        services::ActorService,
    };
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct RecordingPublisher {
        events: Arc<Mutex<Vec<DomainEvent>>>,
    }

    impl EventPublisher for RecordingPublisher {
        fn publish(&self, event: DomainEvent) {
            self.events.lock().expect("event log poisoned").push(event);
        }
    }

    #[test]
    fn disconnect_publishes_actor_disconnected_after_save() {
        let publisher = RecordingPublisher::default();
        let events = publisher.events.clone();
        let feature =
            ActorFeature::new(ActorService::new(InMemoryActorRepository::new()), publisher);
        let snapshot = ActorSnapshot::new("76561198000000000", "Tester");

        let actor = feature
            .disconnect(snapshot)
            .expect("disconnect snapshot should save");

        assert_eq!(actor.uid, "76561198000000000");
        let events = events.lock().expect("event log poisoned");
        assert!(matches!(
            events.as_slice(),
            [DomainEvent::ActorDisconnected(event)] if event.uid == "76561198000000000"
        ));
    }
}
