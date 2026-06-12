use crate::{
    models::Organization,
    shared::{OrganizationError, StorageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait OrganizationRepository: Send + Sync {
    fn find_by_id(&self, id: &str) -> Result<Option<Organization>, OrganizationError>;
    fn find_by_member_uid(&self, uid: &str) -> Result<Option<Organization>, OrganizationError>;
    fn save(&self, organization: Organization) -> Result<Organization, OrganizationError>;
    fn delete(&self, id: &str) -> Result<(), OrganizationError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryOrganizationRepository {
    organizations: Arc<RwLock<HashMap<String, Organization>>>,
}

impl InMemoryOrganizationRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OrganizationRepository for InMemoryOrganizationRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<Organization>, OrganizationError> {
        let organizations = self.organizations.read().map_storage_error()?;
        Ok(organizations.get(id).cloned())
    }

    fn find_by_member_uid(&self, uid: &str) -> Result<Option<Organization>, OrganizationError> {
        let organizations = self.organizations.read().map_storage_error()?;
        Ok(organizations
            .values()
            .find(|organization| organization.has_member(uid))
            .cloned())
    }

    fn save(&self, organization: Organization) -> Result<Organization, OrganizationError> {
        let mut organizations = self.organizations.write().map_storage_error()?;
        organizations.insert(organization.id.clone(), organization.clone());
        Ok(organization)
    }

    fn delete(&self, id: &str) -> Result<(), OrganizationError> {
        let mut organizations = self.organizations.write().map_storage_error()?;
        organizations.remove(id);
        Ok(())
    }
}
