use super::LockerFeature;
use forge_lib::{models::PlayerLocker, repositories::LockerRepository, shared::LockerError};

impl<R> LockerFeature<R>
where
    R: LockerRepository,
{
    pub(crate) fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError> {
        self.service.save(locker)
    }
}
