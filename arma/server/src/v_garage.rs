use crate::log;
use arma_rs::Group;
use forge_lib::{
    models::PlayerVGarage, repositories::InMemoryVGarageRepository, services::VGarageService,
};
use std::sync::LazyLock;

static V_GARAGE_SERVICE: LazyLock<VGarageService<InMemoryVGarageRepository>> =
    LazyLock::new(|| VGarageService::new(InMemoryVGarageRepository::new()));

pub(crate) fn service() -> &'static VGarageService<InMemoryVGarageRepository> {
    &V_GARAGE_SERVICE
}

pub fn group() -> Group {
    Group::new()
        .command("get", get_garage)
        .command("save", save_garage)
        .command("delete", delete_garage)
}

fn get_garage(uid: String) -> String {
    match V_GARAGE_SERVICE.get(&uid) {
        Ok(Some(garage)) => serialize_garage(&garage),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get virtual garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn save_garage(garage_json: String) -> String {
    let garage = match serde_json::from_str::<PlayerVGarage>(&garage_json) {
        Ok(garage) => garage,
        Err(error) => {
            log::error(format_args!("invalid virtual garage payload: {error}"));
            return format!("Error: invalid virtual garage payload: {error}");
        }
    };

    match V_GARAGE_SERVICE.save(garage) {
        Ok(garage) => serialize_garage(&garage),
        Err(error) => {
            log::error(format_args!("failed to save virtual garage: {error}"));
            format!("Error: {error}")
        }
    }
}

fn delete_garage(uid: String) -> String {
    match V_GARAGE_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!(
                "failed to delete virtual garage {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

fn serialize_garage(garage: &PlayerVGarage) -> String {
    match serde_json::to_string(garage) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize virtual garage: {error}"));
            format!("Error: failed to serialize virtual garage: {error}")
        }
    }
}
