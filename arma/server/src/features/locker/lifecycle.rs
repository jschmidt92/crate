use super::LockerFeature;
use forge_lib::{models::PlayerLocker, repositories::LockerRepository, shared::LockerError};

impl<R> LockerFeature<R>
where
    R: LockerRepository,
{
    pub(crate) fn init(&self, uid: &str) -> Result<PlayerLocker, LockerError> {
        self.service.create_actor_locker(uid)
    }

    pub(crate) fn disconnect(&self, uid: &str) -> Result<(), LockerError> {
        self.service.disconnect(uid)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), LockerError> {
        self.service.delete(uid)
    }
}
