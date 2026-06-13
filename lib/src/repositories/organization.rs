use crate::{
    models::{Organization, OrganizationInvite, OrganizationInviteStatus},
    shared::{OrganizationError, StorageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait OrganizationRepository: Send + Sync {
    fn find_by_id(&self, id: &str) -> Result<Option<Organization>, OrganizationError>;
    fn find_by_member_uid(&self, uid: &str) -> Result<Option<Organization>, OrganizationError>;
    fn find_invite(&self, id: &str) -> Result<Option<OrganizationInvite>, OrganizationError>;
    fn find_pending_invite(
        &self,
        organization_id: &str,
        invitee_uid: &str,
    ) -> Result<Option<OrganizationInvite>, OrganizationError>;
    fn save(&self, organization: Organization) -> Result<Organization, OrganizationError>;
    fn save_invite(
        &self,
        invite: OrganizationInvite,
    ) -> Result<OrganizationInvite, OrganizationError>;
    fn delete(&self, id: &str) -> Result<(), OrganizationError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryOrganizationRepository {
    organizations: Arc<RwLock<HashMap<String, Organization>>>,
    invites: Arc<RwLock<HashMap<String, OrganizationInvite>>>,
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

    fn find_invite(&self, id: &str) -> Result<Option<OrganizationInvite>, OrganizationError> {
        let invites = self.invites.read().map_storage_error()?;
        Ok(invites.get(id).cloned())
    }

    fn find_pending_invite(
        &self,
        organization_id: &str,
        invitee_uid: &str,
    ) -> Result<Option<OrganizationInvite>, OrganizationError> {
        let invites = self.invites.read().map_storage_error()?;
        Ok(invites
            .values()
            .find(|invite| {
                invite.organization_id == organization_id
                    && invite.invitee_uid == invitee_uid
                    && invite.status == OrganizationInviteStatus::Pending
            })
            .cloned())
    }

    fn save(&self, organization: Organization) -> Result<Organization, OrganizationError> {
        let mut organizations = self.organizations.write().map_storage_error()?;
        organizations.insert(organization.id.clone(), organization.clone());
        Ok(organization)
    }

    fn save_invite(
        &self,
        invite: OrganizationInvite,
    ) -> Result<OrganizationInvite, OrganizationError> {
        let mut invites = self.invites.write().map_storage_error()?;
        invites.insert(invite.id.to_string(), invite.clone());
        Ok(invite)
    }

    fn delete(&self, id: &str) -> Result<(), OrganizationError> {
        let mut organizations = self.organizations.write().map_storage_error()?;
        organizations.remove(id);
        Ok(())
    }
}
