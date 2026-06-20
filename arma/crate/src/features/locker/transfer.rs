use super::LockerFeature;
use forge_lib::{
    events::EventPublisher,
    models::{DomainEvent, LockerTransferCommitted, PlayerLocker},
    repositories::LockerRepository,
    shared::LockerError,
};

impl<R, E> LockerFeature<R, E>
where
    R: LockerRepository,
    E: EventPublisher,
{
    pub(crate) fn commit_transfer(
        &self,
        locker: PlayerLocker,
    ) -> Result<PlayerLocker, LockerError> {
        let locker = self.service.save(locker)?;
        let distinct_items = locker.locker.items.len();
        let total_quantity = locker
            .locker
            .items
            .values()
            .map(|item| u64::from(item.amount))
            .sum();
        self.events.publish(DomainEvent::LockerTransferCommitted(
            LockerTransferCommitted::new(&locker.uid, distinct_items, total_quantity),
        ));
        Ok(locker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_lib::{
        events::EventPublisher,
        models::{LockerItem, PlayerLocker},
        repositories::InMemoryLockerRepository,
        services::LockerService,
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
    fn committed_transfer_saves_locker_and_publishes_summary_event() {
        let publisher = RecordingPublisher::default();
        let events = publisher.events.clone();
        let feature = LockerFeature::new(
            LockerService::new(InMemoryLockerRepository::new()),
            publisher,
        );
        let mut locker = PlayerLocker::new("76561198000000000");
        locker
            .locker
            .add_item(LockerItem::new("items", "FirstAidKit", 3).expect("valid item"))
            .expect("item should be added");

        feature
            .commit_transfer(locker)
            .expect("transfer should commit");

        let events = events.lock().expect("event log poisoned");
        assert!(matches!(
            events.as_slice(),
            [DomainEvent::LockerTransferCommitted(event)]
                if event.uid == "76561198000000000"
                    && event.distinct_items == 1
                    && event.total_quantity == 3
        ));
    }
}
