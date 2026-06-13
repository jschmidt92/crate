use crate::log;
use arma_rs::Group;
use forge_lib::{models::PlayerGarage, services::GarageService};
use std::sync::LazyLock;

static GARAGE_SERVICE: LazyLock<GarageService<crate::persistence::CachedGarageRepository>> =
    LazyLock::new(|| GarageService::new(crate::persistence::garage_repository()));

pub fn group() -> Group {
    Group::new()
        .command("init", init_garage)
        .command("disconnect", disconnect_garage)
        .command("get", get_garage)
        .command("save", save_garage)
        .command("delete", delete_garage)
}

fn init_garage(uid: String) -> String {
    match GARAGE_SERVICE.create_actor_garage(&uid) {
        Ok(garage) => serialize_garage(&garage),
        Err(error) => {
            log::error(format_args!("failed to init garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn disconnect_garage(uid: String) -> String {
    match GARAGE_SERVICE.disconnect(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to disconnect garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn get_garage(uid: String) -> String {
    match GARAGE_SERVICE.get(&uid) {
        Ok(Some(garage)) => serialize_garage(&garage),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn save_garage(garage_json: String) -> String {
    let garage = match serde_json::from_str::<PlayerGarage>(&garage_json) {
        Ok(garage) => garage,
        Err(error) => {
            log::error(format_args!("invalid garage payload: {error}"));
            return format!("Error: invalid garage payload: {error}");
        }
    };

    match GARAGE_SERVICE.save(garage) {
        Ok(garage) => serialize_garage(&garage),
        Err(error) => {
            log::error(format_args!("failed to save garage: {error}"));
            format!("Error: {error}")
        }
    }
}

fn delete_garage(uid: String) -> String {
    match GARAGE_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn serialize_garage(garage: &PlayerGarage) -> String {
    match serde_json::to_string(garage) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize garage: {error}"));
            format!("Error: failed to serialize garage: {error}")
        }
    }
}
