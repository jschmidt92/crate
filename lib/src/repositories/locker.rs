use crate::{
    models::PlayerLocker,
    shared::{LockerError, StorageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait LockerRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError>;
    fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError>;
    fn delete(&self, uid: &str) -> Result<(), LockerError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryLockerRepository {
    lockers: Arc<RwLock<HashMap<String, PlayerLocker>>>,
}

impl InMemoryLockerRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LockerRepository for InMemoryLockerRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError> {
        let lockers = self.lockers.read().map_storage_error()?;
        Ok(lockers.get(uid).cloned())
    }

    fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError> {
        let mut lockers = self.lockers.write().map_storage_error()?;
        locker.locker.validate()?;
        lockers.insert(locker.uid.clone(), locker.clone());
        Ok(locker)
    }

    fn delete(&self, uid: &str) -> Result<(), LockerError> {
        let mut lockers = self.lockers.write().map_storage_error()?;
        lockers.remove(uid);
        Ok(())
    }
}
