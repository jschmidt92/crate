use crate::{events::ServerEventPublisher, features::locker::LockerFeature, log, response};
use arma_rs::Group;
use forge_lib::{models::PlayerLocker, services::LockerService};
use std::sync::LazyLock;

static LOCKER_FEATURE: LazyLock<
    LockerFeature<crate::persistence::CachedLockerRepository, ServerEventPublisher>,
> = LazyLock::new(|| {
    LockerFeature::new(
        LockerService::new(crate::persistence::locker_repository()),
        ServerEventPublisher,
    )
});

pub fn group() -> Group {
    Group::new()
        .command("init", init_locker)
        .command("get", get_locker)
        .command("save", save_locker)
        .command("delete", delete_locker)
}

pub(crate) fn init_locker(uid: String) -> String {
    match LOCKER_FEATURE.init(&uid) {
        Ok(locker) => response::json(&locker, "locker"),
        Err(error) => {
            log::error(format_args!("failed to init locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_locker(uid: String) -> String {
    match LOCKER_FEATURE.get(&uid) {
        Ok(Some(locker)) => response::json(&locker, "locker"),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn save_locker(locker_json: String) -> String {
    let locker = match serde_json::from_str::<PlayerLocker>(&locker_json) {
        Ok(locker) => locker,
        Err(error) => {
            log::error(format_args!("invalid locker payload: {error}"));
            return format!("Error: invalid locker payload: {error}");
        }
    };

    match LOCKER_FEATURE.save(locker) {
        Ok(locker) => response::json(&locker, "locker"),
        Err(error) => {
            log::error(format_args!("failed to save locker: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn commit_locker(locker_json: String) -> String {
    let locker = match serde_json::from_str::<PlayerLocker>(&locker_json) {
        Ok(locker) => locker,
        Err(error) => {
            log::error(format_args!("invalid locker commit payload: {error}"));
            return format!("Error: invalid locker commit payload: {error}");
        }
    };
    match LOCKER_FEATURE.commit_transfer(locker) {
        Ok(locker) => response::json(&locker, "locker transfer"),
        Err(error) => {
            log::error(format_args!("failed to commit locker transfer: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_locker(uid: String) -> String {
    match LOCKER_FEATURE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
