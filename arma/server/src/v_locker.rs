use crate::{features::v_locker::VLockerFeature, log};
use arma_rs::Group;
use forge_lib::{
    models::{PlayerVLocker, VLocker},
    services::VLockerService,
};
use std::sync::LazyLock;

static V_LOCKER_FEATURE: LazyLock<VLockerFeature<crate::persistence::CachedVLockerRepository>> =
    LazyLock::new(|| {
        VLockerFeature::new(VLockerService::new(
            crate::persistence::v_locker_repository(),
        ))
    });

pub fn group() -> Group {
    Group::new()
        .command("init", init_locker)
        .command("disconnect", disconnect_locker)
        .command("get", get_locker)
        .command("save", save_locker)
        .command("delete", delete_locker)
}

pub(crate) fn init_locker(uid: String, unlocks_json: String) -> String {
    let unlocks = match serde_json::from_str::<VLocker>(&unlocks_json) {
        Ok(unlocks) => unlocks,
        Err(error) => {
            log::error(format_args!(
                "invalid virtual locker unlock payload: {error}"
            ));
            return format!("Error: invalid virtual locker unlock payload: {error}");
        }
    };

    match V_LOCKER_FEATURE.init(&uid, &unlocks) {
        Ok(locker) => serialize_locker(&locker),
        Err(error) => {
            log::error(format_args!("failed to init virtual locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_locker(uid: String) -> String {
    match V_LOCKER_FEATURE.disconnect(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!(
                "failed to disconnect virtual locker {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_locker(uid: String) -> String {
    match V_LOCKER_FEATURE.get(&uid) {
        Ok(Some(locker)) => serialize_locker(&locker),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get virtual locker {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn save_locker(locker_json: String) -> String {
    let locker = match serde_json::from_str::<PlayerVLocker>(&locker_json) {
        Ok(locker) => locker,
        Err(error) => {
            log::error(format_args!("invalid virtual locker payload: {error}"));
            return format!("Error: invalid virtual locker payload: {error}");
        }
    };

    match V_LOCKER_FEATURE.save(locker) {
        Ok(locker) => serialize_locker(&locker),
        Err(error) => {
            log::error(format_args!("failed to save virtual locker: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_locker(uid: String) -> String {
    match V_LOCKER_FEATURE.delete(&uid) {
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
    serde_json::to_string(locker).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize virtual locker: {error}"));
        format!("Error: failed to serialize virtual locker: {error}")
    })
}
