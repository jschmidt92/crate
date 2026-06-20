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
    pub(crate) fn get(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError> {
        self.service.get(uid)
    }
}
