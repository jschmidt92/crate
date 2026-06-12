use crate::{
    models::PlayerBankProfile,
    shared::{BankError, StorageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait BankRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerBankProfile>, BankError>;
    fn save(&self, profile: PlayerBankProfile) -> Result<PlayerBankProfile, BankError>;
    fn delete(&self, uid: &str) -> Result<(), BankError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryBankRepository {
    profiles: Arc<RwLock<HashMap<String, PlayerBankProfile>>>,
}

impl InMemoryBankRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BankRepository for InMemoryBankRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerBankProfile>, BankError> {
        let profiles = self.profiles.read().map_storage_error()?;
        Ok(profiles.get(uid).cloned())
    }

    fn save(&self, profile: PlayerBankProfile) -> Result<PlayerBankProfile, BankError> {
        let mut profiles = self.profiles.write().map_storage_error()?;
        profiles.insert(profile.uid.clone(), profile.clone());
        Ok(profile)
    }

    fn delete(&self, uid: &str) -> Result<(), BankError> {
        let mut profiles = self.profiles.write().map_storage_error()?;
        profiles.remove(uid);
        Ok(())
    }
}
