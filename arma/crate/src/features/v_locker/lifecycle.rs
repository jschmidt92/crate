use super::VLockerFeature;
use forge_lib::{
    models::{PlayerVLocker, VLocker},
    repositories::VLockerRepository,
    shared::VLockerError,
};

impl<R> VLockerFeature<R>
where
    R: VLockerRepository,
{
    pub(crate) fn init(&self, uid: &str, unlocks: &VLocker) -> Result<PlayerVLocker, VLockerError> {
        self.service.create_actor_locker(uid, unlocks)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), VLockerError> {
        self.service.delete(uid)
    }
}
