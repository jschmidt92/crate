use crate::log;
use arma_rs::Group;
use forge_lib::{
    models::PlayerVLocker, repositories::InMemoryVLockerRepository, services::VLockerService,
};
use std::sync::LazyLock;

static V_LOCKER_SERVICE: LazyLock<VLockerService<InMemoryVLockerRepository>> =
    LazyLock::new(|| VLockerService::new(InMemoryVLockerRepository::new()));

pub(crate) fn service() -> &'static VLockerService<InMemoryVLockerRepository> {
    &V_LOCKER_SERVICE
}

pub fn group() -> Group {
    Group::new()
        .command("get", get_locker)
        .command("save", save_locker)
        .command("delete", delete_locker)
}

fn get_locker(uid: String) -> String {
    match V_LOCKER_SERVICE.get(&uid) {
        Ok(Some(locker)) => serialize_locker(&locker),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get virtual locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn save_locker(locker_json: String) -> String {
    let locker = match serde_json::from_str::<PlayerVLocker>(&locker_json) {
        Ok(locker) => locker,
        Err(error) => {
            log::error(format_args!("invalid virtual locker payload: {error}"));
            return format!("Error: invalid virtual locker payload: {error}");
        }
    };

    match V_LOCKER_SERVICE.save(locker) {
        Ok(locker) => serialize_locker(&locker),
        Err(error) => {
            log::error(format_args!("failed to save virtual locker: {error}"));
            format!("Error: {error}")
        }
    }
}

fn delete_locker(uid: String) -> String {
    match V_LOCKER_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!(
                "failed to delete virtual locker {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

fn serialize_locker(locker: &PlayerVLocker) -> String {
    match serde_json::to_string(locker) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize virtual locker: {error}"));
            format!("Error: failed to serialize virtual locker: {error}")
        }
    }
}
