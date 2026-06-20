use super::LockerFeature;
use forge_lib::{
    events::EventPublisher, models::PlayerLocker, repositories::LockerRepository,
    shared::LockerError,
};

impl<R, E> LockerFeature<R, E>
where
    R: LockerRepository,
    E: EventPublisher,
{
    pub(crate) fn init(&self, uid: &str) -> Result<PlayerLocker, LockerError> {
        self.service.create_actor_locker(uid)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), LockerError> {
        self.service.delete(uid)
    }
}
