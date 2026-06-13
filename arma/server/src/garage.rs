use crate::{features::garage::GarageFeature, log, response};
use arma_rs::Group;
use forge_lib::{models::PlayerGarage, services::GarageService};
use std::sync::LazyLock;

static GARAGE_FEATURE: LazyLock<GarageFeature<crate::persistence::CachedGarageRepository>> =
    LazyLock::new(|| {
        GarageFeature::new(GarageService::new(crate::persistence::garage_repository()))
    });

pub fn group() -> Group {
    Group::new()
        .command("init", init_garage)
        .command("disconnect", disconnect_garage)
        .command("get", get_garage)
        .command("save", save_garage)
        .command("delete", delete_garage)
}

pub(crate) fn init_garage(uid: String) -> String {
    match GARAGE_FEATURE.init(&uid) {
        Ok(garage) => response::json(&garage, "garage"),
        Err(error) => {
            log::error(format_args!("failed to init garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_garage(uid: String) -> String {
    match GARAGE_FEATURE.disconnect(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to disconnect garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_garage(uid: String) -> String {
    match GARAGE_FEATURE.get(&uid) {
        Ok(Some(garage)) => response::json(&garage, "garage"),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn save_garage(garage_json: String) -> String {
    let garage = match serde_json::from_str::<PlayerGarage>(&garage_json) {
        Ok(garage) => garage,
        Err(error) => {
            log::error(format_args!("invalid garage payload: {error}"));
            return format!("Error: invalid garage payload: {error}");
        }
    };

    match GARAGE_FEATURE.save(garage) {
        Ok(garage) => response::json(&garage, "garage"),
        Err(error) => {
            log::error(format_args!("failed to save garage: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_garage(uid: String) -> String {
    match GARAGE_FEATURE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
