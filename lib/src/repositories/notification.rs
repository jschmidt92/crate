use crate::{
    models::Notification,
    shared::{NotificationError, StorageError},
};
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub trait NotificationRepository: Send + Sync {
    fn list_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError>;
    fn list_unread_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>, NotificationError>;
    fn save(&self, notification: Notification) -> Result<Notification, NotificationError>;
    fn mark_read(&self, uid: &str, id: Uuid) -> Result<Notification, NotificationError>;
    fn mark_all_read(&self, uid: &str) -> Result<Vec<Notification>, NotificationError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryNotificationRepository {
    notifications: Arc<RwLock<HashMap<Uuid, Notification>>>,
}

impl InMemoryNotificationRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NotificationRepository for InMemoryNotificationRepository {
    fn list_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        let notifications = self.notifications.read().map_storage_error()?;
        Ok(sorted_notifications(
            notifications
                .values()
                .filter(|notification| notification.uid == uid)
                .cloned()
                .collect(),
        ))
    }

    fn list_unread_by_uid(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        let notifications = self.notifications.read().map_storage_error()?;
        Ok(sorted_notifications(
            notifications
                .values()
                .filter(|notification| notification.uid == uid && notification.read_at.is_none())
                .cloned()
                .collect(),
        ))
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>, NotificationError> {
        let notifications = self.notifications.read().map_storage_error()?;
        Ok(notifications.get(&id).cloned())
    }

    fn save(&self, notification: Notification) -> Result<Notification, NotificationError> {
        let mut notifications = self.notifications.write().map_storage_error()?;
        notifications.insert(notification.id, notification.clone());
        Ok(notification)
    }

    fn mark_read(&self, uid: &str, id: Uuid) -> Result<Notification, NotificationError> {
        let mut notifications = self.notifications.write().map_storage_error()?;
        let notification = notifications
            .get_mut(&id)
            .ok_or(NotificationError::NotFound)?;
        if notification.uid != uid {
            return Err(NotificationError::NotFound);
        }
        if notification.read_at.is_none() {
            notification.read_at = Some(Utc::now());
        }
        Ok(notification.clone())
    }

    fn mark_all_read(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        let mut notifications = self.notifications.write().map_storage_error()?;
        let now = Utc::now();
        let mut updated = Vec::new();
        for notification in notifications.values_mut() {
            if notification.uid == uid && notification.read_at.is_none() {
                notification.read_at = Some(now);
                updated.push(notification.clone());
            }
        }
        Ok(sorted_notifications(updated))
    }
}

fn sorted_notifications(mut notifications: Vec<Notification>) -> Vec<Notification> {
    notifications.sort_by(|a, b| {
        b.created_at
            .cmp(&a.created_at)
            .then_with(|| b.id.cmp(&a.id))
    });
    notifications
}
