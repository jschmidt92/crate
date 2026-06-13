use super::{enqueue_delete, enqueue_upsert};
use forge_lib::models::{Notification, PlayerGarage, PlayerLocker, PlayerVGarage, PlayerVLocker};
use forge_lib::{
    models::{Actor, Organization, OrganizationInvite, PlayerBankProfile},
    repositories::{
        ActorRepository, BankRepository, GarageRepository, InMemoryActorRepository,
        InMemoryBankRepository, InMemoryGarageRepository, InMemoryLockerRepository,
        InMemoryNotificationRepository, InMemoryOrganizationRepository, InMemoryVGarageRepository,
        InMemoryVLockerRepository, LockerRepository, NotificationRepository,
        OrganizationRepository, VGarageRepository, VLockerRepository,
    },
    shared::{
        ActorError, BankError, GarageError, LockerError, NotificationError, OrganizationError,
        VGarageError, VLockerError,
    },
};

#[derive(Clone)]
pub struct CachedActorRepository {
    cache: InMemoryActorRepository,
}

impl CachedActorRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryActorRepository::new(),
        }
    }
}

impl ActorRepository for CachedActorRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<Actor>, ActorError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, actor: Actor) -> Result<Actor, ActorError> {
        let actor = self.cache.save(actor)?;
        enqueue_upsert("actor", &actor.uid, &actor);
        Ok(actor)
    }

    fn delete(&self, uid: &str) -> Result<(), ActorError> {
        self.cache.delete(uid)?;
        enqueue_delete("actor", uid);
        Ok(())
    }
}

pub(super) fn cache_actor(repository: &CachedActorRepository, actor: Actor) {
    let _ = repository.cache.save(actor);
}

#[derive(Clone)]
pub struct CachedBankRepository {
    cache: InMemoryBankRepository,
}

impl CachedBankRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryBankRepository::new(),
        }
    }

    pub(crate) fn save_without_enqueue(
        &self,
        profile: PlayerBankProfile,
    ) -> Result<PlayerBankProfile, BankError> {
        self.cache.save(profile)
    }
}

impl BankRepository for CachedBankRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerBankProfile>, BankError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, profile: PlayerBankProfile) -> Result<PlayerBankProfile, BankError> {
        let profile = self.cache.save(profile)?;
        enqueue_upsert("bank", &profile.uid, &profile);
        Ok(profile)
    }

    fn delete(&self, uid: &str) -> Result<(), BankError> {
        self.cache.delete(uid)?;
        enqueue_delete("bank", uid);
        Ok(())
    }
}

pub(super) fn cache_bank_profile(repository: &CachedBankRepository, profile: PlayerBankProfile) {
    let _ = repository.cache.save(profile);
}

#[derive(Clone)]
pub struct CachedGarageRepository {
    cache: InMemoryGarageRepository,
}

impl CachedGarageRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryGarageRepository::new(),
        }
    }
}

impl GarageRepository for CachedGarageRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerGarage>, GarageError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, garage: PlayerGarage) -> Result<PlayerGarage, GarageError> {
        let garage = self.cache.save(garage)?;
        enqueue_upsert("garage", &garage.uid, &garage);
        Ok(garage)
    }

    fn delete(&self, uid: &str) -> Result<(), GarageError> {
        self.cache.delete(uid)?;
        enqueue_delete("garage", uid);
        Ok(())
    }
}

pub(super) fn cache_garage(repository: &CachedGarageRepository, garage: PlayerGarage) {
    let _ = repository.cache.save(garage);
}

#[derive(Clone)]
pub struct CachedLockerRepository {
    cache: InMemoryLockerRepository,
}

impl CachedLockerRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryLockerRepository::new(),
        }
    }
}

impl LockerRepository for CachedLockerRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerLocker>, LockerError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, locker: PlayerLocker) -> Result<PlayerLocker, LockerError> {
        let locker = self.cache.save(locker)?;
        enqueue_upsert("locker", &locker.uid, &locker);
        Ok(locker)
    }

    fn delete(&self, uid: &str) -> Result<(), LockerError> {
        self.cache.delete(uid)?;
        enqueue_delete("locker", uid);
        Ok(())
    }
}

pub(super) fn cache_locker(repository: &CachedLockerRepository, locker: PlayerLocker) {
    let _ = repository.cache.save(locker);
}

#[derive(Clone)]
pub struct CachedNotificationRepository {
    cache: InMemoryNotificationRepository,
}

impl CachedNotificationRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryNotificationRepository::new(),
        }
    }

    pub(crate) fn save_without_enqueue(
        &self,
        notification: Notification,
    ) -> Result<Notification, NotificationError> {
        self.cache.save(notification)
    }
}

impl NotificationRepository for CachedNotificationRepository {
    fn list_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        self.cache.list_by_uid(uid)
    }

    fn list_unread_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        self.cache.list_unread_by_uid(uid)
    }

    fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Notification>, NotificationError> {
        self.cache.find_by_id(id)
    }

    fn save(&self, notification: Notification) -> Result<Notification, NotificationError> {
        let notification = self.cache.save(notification)?;
        enqueue_upsert("notification", &notification.id.to_string(), &notification);
        Ok(notification)
    }

    fn mark_read(&self, uid: &str, id: uuid::Uuid) -> Result<Notification, NotificationError> {
        let notification = self.cache.mark_read(uid, id)?;
        enqueue_upsert("notification", &notification.id.to_string(), &notification);
        Ok(notification)
    }

    fn mark_all_read(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        let notifications = self.cache.mark_all_read(uid)?;
        for notification in &notifications {
            enqueue_upsert("notification", &notification.id.to_string(), notification);
        }
        Ok(notifications)
    }
}

pub(super) fn cache_notification(
    repository: &CachedNotificationRepository,
    notification: Notification,
) {
    let _ = repository.cache.save(notification);
}

#[derive(Clone)]
pub struct CachedVGarageRepository {
    cache: InMemoryVGarageRepository,
}

impl CachedVGarageRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryVGarageRepository::new(),
        }
    }
}

impl VGarageRepository for CachedVGarageRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVGarage>, VGarageError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, garage: PlayerVGarage) -> Result<PlayerVGarage, VGarageError> {
        let garage = self.cache.save(garage)?;
        enqueue_upsert("v_garage", &garage.uid, &garage);
        Ok(garage)
    }

    fn delete(&self, uid: &str) -> Result<(), VGarageError> {
        self.cache.delete(uid)?;
        enqueue_delete("v_garage", uid);
        Ok(())
    }
}

pub(super) fn cache_v_garage(repository: &CachedVGarageRepository, garage: PlayerVGarage) {
    let _ = repository.cache.save(garage);
}

#[derive(Clone)]
pub struct CachedVLockerRepository {
    cache: InMemoryVLockerRepository,
}

impl CachedVLockerRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryVLockerRepository::new(),
        }
    }
}

impl VLockerRepository for CachedVLockerRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVLocker>, VLockerError> {
        self.cache.find_by_uid(uid)
    }

    fn save(&self, locker: PlayerVLocker) -> Result<PlayerVLocker, VLockerError> {
        let locker = self.cache.save(locker)?;
        enqueue_upsert("v_locker", &locker.uid, &locker);
        Ok(locker)
    }

    fn delete(&self, uid: &str) -> Result<(), VLockerError> {
        self.cache.delete(uid)?;
        enqueue_delete("v_locker", uid);
        Ok(())
    }
}

pub(super) fn cache_v_locker(repository: &CachedVLockerRepository, locker: PlayerVLocker) {
    let _ = repository.cache.save(locker);
}

#[derive(Clone)]
pub struct CachedOrganizationRepository {
    cache: InMemoryOrganizationRepository,
}

impl CachedOrganizationRepository {
    pub fn new() -> Self {
        Self {
            cache: InMemoryOrganizationRepository::new(),
        }
    }

    pub(crate) fn save_without_enqueue(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationError> {
        self.cache.save(organization)
    }
}

impl OrganizationRepository for CachedOrganizationRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<Organization>, OrganizationError> {
        self.cache.find_by_id(id)
    }

    fn find_by_member_uid(&self, uid: &str) -> Result<Option<Organization>, OrganizationError> {
        self.cache.find_by_member_uid(uid)
    }

    fn find_invite(&self, id: &str) -> Result<Option<OrganizationInvite>, OrganizationError> {
        self.cache.find_invite(id)
    }

    fn find_pending_invite(
        &self,
        organization_id: &str,
        invitee_uid: &str,
    ) -> Result<Option<OrganizationInvite>, OrganizationError> {
        self.cache.find_pending_invite(organization_id, invitee_uid)
    }

    fn save(&self, organization: Organization) -> Result<Organization, OrganizationError> {
        let organization = self.cache.save(organization)?;
        enqueue_upsert("organization", &organization.id, &organization);
        Ok(organization)
    }

    fn save_invite(
        &self,
        invite: OrganizationInvite,
    ) -> Result<OrganizationInvite, OrganizationError> {
        let invite = self.cache.save_invite(invite)?;
        enqueue_upsert("organization_invite", &invite.id.to_string(), &invite);
        Ok(invite)
    }

    fn delete(&self, id: &str) -> Result<(), OrganizationError> {
        self.cache.delete(id)?;
        enqueue_delete("organization", id);
        Ok(())
    }
}

pub(super) fn cache_organization(
    repository: &CachedOrganizationRepository,
    organization: Organization,
) {
    let _ = repository.cache.save(organization);
}

pub(super) fn cache_organization_invite(
    repository: &CachedOrganizationRepository,
    invite: OrganizationInvite,
) {
    let _ = repository.cache.save_invite(invite);
}
