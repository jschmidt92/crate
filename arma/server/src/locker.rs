use crate::log;
use arma_rs::Group;
use forge_lib::{models::PlayerLocker, services::LockerService};
use std::sync::LazyLock;

static LOCKER_SERVICE: LazyLock<LockerService<crate::persistence::CachedLockerRepository>> =
    LazyLock::new(|| LockerService::new(crate::persistence::locker_repository()));

pub fn group() -> Group {
    Group::new()
        .command("init", init_locker)
        .command("disconnect", disconnect_locker)
        .command("get", get_locker)
        .command("save", save_locker)
        .command("delete", delete_locker)
}

pub(crate) fn init_locker(uid: String) -> String {
    match LOCKER_SERVICE.create_actor_locker(&uid) {
        Ok(locker) => serialize_locker(&locker),
        Err(error) => {
            log::error(format_args!("failed to init locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_locker(uid: String) -> String {
    match LOCKER_SERVICE.disconnect(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to disconnect locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_locker(uid: String) -> String {
    match LOCKER_SERVICE.get(&uid) {
        Ok(Some(locker)) => serialize_locker(&locker),
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

    match LOCKER_SERVICE.save(locker) {
        Ok(locker) => serialize_locker(&locker),
        Err(error) => {
            log::error(format_args!("failed to save locker: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_locker(uid: String) -> String {
    match LOCKER_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn serialize_locker(locker: &PlayerLocker) -> String {
    match serde_json::to_string(locker) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize locker: {error}"));
            format!("Error: failed to serialize locker: {error}")
        }
    }
}
