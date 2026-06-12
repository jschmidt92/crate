use crate::{
    models::{PlayerVLocker, VLocker},
    repositories::VLockerRepository,
    shared::VLockerError,
};

#[derive(Clone)]
pub struct VLockerService<R> {
    repository: R,
}

impl<R> VLockerService<R>
where
    R: VLockerRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_actor_locker(
        &self,
        uid: &str,
        starting_unlocks: &VLocker,
    ) -> Result<PlayerVLocker, VLockerError> {
        validate_uid(uid)?;

        if let Some(mut locker) = self.repository.find_by_uid(uid)? {
            locker.unlocks.merge(starting_unlocks);
            return self.repository.save(locker);
        }

        self.repository
            .save(PlayerVLocker::new(uid, starting_unlocks.clone()))
    }

    pub fn get(&self, uid: &str) -> Result<Option<PlayerVLocker>, VLockerError> {
        validate_uid(uid)?;
        self.repository.find_by_uid(uid)
    }

    pub fn save(&self, locker: PlayerVLocker) -> Result<PlayerVLocker, VLockerError> {
        validate_uid(&locker.uid)?;
        self.repository.save(locker)
    }

    pub fn delete(&self, uid: &str) -> Result<(), VLockerError> {
        validate_uid(uid)?;
        self.repository.delete(uid)
    }
}

fn validate_uid(uid: &str) -> Result<(), VLockerError> {
    if uid.trim().is_empty() {
        return Err(VLockerError::InvalidUid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryVLockerRepository;

    #[test]
    fn create_actor_locker_provisions_starting_unlocks() {
        let service = VLockerService::new(InMemoryVLockerRepository::new());
        let starting = VLocker {
            weapons: vec!["hgun_P07_F".to_string()],
            ..VLocker::default()
        };

        let locker = service
            .create_actor_locker("steam:local-dev", &starting)
            .expect("locker should be created");

        assert_eq!(locker.uid, "steam:local-dev");
        assert_eq!(locker.unlocks.weapons, ["hgun_P07_F"]);
    }

    #[test]
    fn create_actor_locker_merges_existing_locker() {
        let service = VLockerService::new(InMemoryVLockerRepository::new());
        let first = VLocker {
            weapons: vec!["hgun_P07_F".to_string()],
            ..VLocker::default()
        };
        let second = VLocker {
            weapons: vec!["hgun_P07_F".to_string(), "arifle_MX_F".to_string()],
            ..VLocker::default()
        };

        service
            .create_actor_locker("steam:local-dev", &first)
            .expect("locker should be created");
        let locker = service
            .create_actor_locker("steam:local-dev", &second)
            .expect("locker should be updated");

        assert_eq!(locker.unlocks.weapons, ["hgun_P07_F", "arifle_MX_F"]);
    }
}
