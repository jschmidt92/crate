use super::LockerFeature;
use forge_lib::{models::PlayerLocker, repositories::LockerRepository, shared::LockerError};

impl<R> LockerFeature<R>
where
    R: LockerRepository,
{
    pub(crate) fn get(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError> {
        self.service.get(uid)
    }
}
