use super::VLockerFeature;
use forge_lib::{models::PlayerVLocker, repositories::VLockerRepository, shared::VLockerError};

impl<R> VLockerFeature<R>
where
    R: VLockerRepository,
{
    pub(crate) fn save(&self, locker: PlayerVLocker) -> Result<PlayerVLocker, VLockerError> {
        self.service.save(locker)
    }
}
