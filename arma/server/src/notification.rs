use crate::{features::notification::NotificationFeature, log, response};
use arma_rs::Group;
use forge_lib::services::NotificationService;
use std::sync::LazyLock;

static NOTIFICATION_FEATURE: LazyLock<
    NotificationFeature<crate::persistence::CachedNotificationRepository>,
> = LazyLock::new(|| {
    NotificationFeature::new(NotificationService::new(
        crate::persistence::notification_repository(),
    ))
});

pub fn group() -> Group {
    Group::new()
        .command("list", list_notifications)
        .command("unread", unread_notifications)
        .command("mark_read", mark_read_notification)
        .command("mark_all_read", mark_all_read_notifications)
}

pub(crate) fn list_notifications(uid: String) -> String {
    match NOTIFICATION_FEATURE.list(&uid) {
        Ok(notifications) => response::json(&notifications, "notifications"),
        Err(error) => {
            log::error(format_args!(
                "failed to list notifications for {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn unread_notifications(uid: String) -> String {
    match NOTIFICATION_FEATURE.unread(&uid) {
        Ok(notifications) => response::json(&notifications, "unread notifications"),
        Err(error) => {
            log::error(format_args!(
                "failed to list unread notifications for {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn mark_read_notification(uid: String, id: String) -> String {
    match NOTIFICATION_FEATURE.mark_read(&uid, &id) {
        Ok(notification) => response::json(&notification, "notification"),
        Err(error) => {
            log::error(format_args!(
                "failed to mark notification {id} read for {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn mark_all_read_notifications(uid: String) -> String {
    match NOTIFICATION_FEATURE.mark_all_read(&uid) {
        Ok(notifications) => response::json(&notifications, "notifications"),
        Err(error) => {
            log::error(format_args!(
                "failed to mark all notifications read for {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}
