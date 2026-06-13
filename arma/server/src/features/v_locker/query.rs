use super::VLockerFeature;
use forge_lib::{models::PlayerVLocker, repositories::VLockerRepository, shared::VLockerError};

impl<R> VLockerFeature<R>
where
    R: VLockerRepository,
{
    pub(crate) fn get(&self, uid: &str) -> Result<Option<PlayerVLocker>, VLockerError> {
        self.service.get(uid)
    }
}
