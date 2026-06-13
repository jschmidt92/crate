use crate::{
    models::Notification, repositories::NotificationRepository, shared::NotificationError,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct NotificationService<R> {
    repository: R,
}

impl<R> NotificationService<R>
where
    R: NotificationRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn list(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        validate_uid(uid)?;
        self.repository.list_by_uid(uid)
    }

    pub fn unread(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        validate_uid(uid)?;
        self.repository.list_unread_by_uid(uid)
    }

    pub fn mark_read(&self, uid: &str, id: &str) -> Result<Notification, NotificationError> {
        validate_uid(uid)?;
        let id = parse_id(id)?;
        self.repository.mark_read(uid, id)
    }

    pub fn mark_all_read(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        validate_uid(uid)?;
        self.repository.mark_all_read(uid)
    }
}

fn validate_uid(uid: &str) -> Result<(), NotificationError> {
    if uid.trim().is_empty() {
        return Err(NotificationError::InvalidUid);
    }
    Ok(())
}

fn parse_id(id: &str) -> Result<Uuid, NotificationError> {
    Uuid::parse_str(id).map_err(|_| NotificationError::InvalidId)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{Notification, NotificationKind},
        repositories::InMemoryNotificationRepository,
    };

    #[test]
    fn unread_only_returns_unread_notifications_for_uid() {
        let repository = InMemoryNotificationRepository::new();
        repository
            .save(Notification::new(
                "steam:local-dev",
                NotificationKind::OrganizationInvite,
                "Invite",
                "Join us",
            ))
            .expect("notification should save");
        repository
            .save(Notification::new(
                "steam:other",
                NotificationKind::OrganizationInvite,
                "Invite",
                "Join us",
            ))
            .expect("notification should save");
        let service = NotificationService::new(repository);

        let notifications = service
            .unread("steam:local-dev")
            .expect("unread lookup should succeed");

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].uid, "steam:local-dev");
    }

    #[test]
    fn mark_read_sets_read_at() {
        let repository = InMemoryNotificationRepository::new();
        let notification = repository
            .save(Notification::new(
                "steam:local-dev",
                NotificationKind::OrganizationInvite,
                "Invite",
                "Join us",
            ))
            .expect("notification should save");
        let service = NotificationService::new(repository);

        let notification = service
            .mark_read("steam:local-dev", &notification.id.to_string())
            .expect("notification should mark read");

        assert!(notification.read_at.is_some());
    }
}
