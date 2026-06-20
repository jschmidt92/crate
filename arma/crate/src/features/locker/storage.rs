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
    pub(crate) fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError> {
        self.service.save(locker)
    }
}
