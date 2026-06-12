use crate::{
    models::PlayerVLocker,
    shared::{StorageError, VLockerError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait VLockerRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVLocker>, VLockerError>;
    fn save(&self, locker: PlayerVLocker) -> Result<PlayerVLocker, VLockerError>;
    fn delete(&self, uid: &str) -> Result<(), VLockerError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryVLockerRepository {
    lockers: Arc<RwLock<HashMap<String, PlayerVLocker>>>,
}

impl InMemoryVLockerRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VLockerRepository for InMemoryVLockerRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVLocker>, VLockerError> {
        let lockers = self.lockers.read().map_storage_error()?;

        Ok(lockers.get(uid).cloned())
    }

    fn save(&self, locker: PlayerVLocker) -> Result<PlayerVLocker, VLockerError> {
        let mut lockers = self.lockers.write().map_storage_error()?;

        lockers.insert(locker.uid.clone(), locker.clone());
        Ok(locker)
    }

    fn delete(&self, uid: &str) -> Result<(), VLockerError> {
        let mut lockers = self.lockers.write().map_storage_error()?;

        lockers.remove(uid);
        Ok(())
    }
}
