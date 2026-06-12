use crate::{models::PlayerLocker, repositories::LockerRepository, shared::LockerError};

#[derive(Clone)]
pub struct LockerService<R> {
    repository: R,
}

impl<R> LockerService<R>
where
    R: LockerRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_actor_locker(&self, uid: &str) -> Result<PlayerLocker, LockerError> {
        validate_uid(uid)?;

        if let Some(locker) = self.repository.find_by_uid(uid)? {
            return Ok(locker);
        }

        self.repository.save(PlayerLocker::new(uid))
    }

    pub fn get(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError> {
        validate_uid(uid)?;
        self.repository.find_by_uid(uid)
    }

    pub fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError> {
        validate_uid(&locker.uid)?;
        self.repository.save(locker)
    }

    pub fn delete(&self, uid: &str) -> Result<(), LockerError> {
        validate_uid(uid)?;
        self.repository.delete(uid)
    }

    pub fn disconnect(&self, uid: &str) -> Result<(), LockerError> {
        validate_uid(uid)?;
        Ok(())
    }
}

fn validate_uid(uid: &str) -> Result<(), LockerError> {
    if uid.trim().is_empty() {
        return Err(LockerError::InvalidUid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryLockerRepository;

    #[test]
    fn create_actor_locker_creates_empty_locker() {
        let service = LockerService::new(InMemoryLockerRepository::new());

        let locker = service
            .create_actor_locker("steam:local-dev")
            .expect("locker should be created");

        assert_eq!(locker.uid, "steam:local-dev");
        assert!(locker.locker.items.is_empty());
    }
}
