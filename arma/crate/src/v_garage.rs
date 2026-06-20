use crate::{features::v_garage::VGarageFeature, log, response};
use arma_rs::Group;
use forge_lib::{
    models::{PlayerVGarage, VGarage},
    services::VGarageService,
};
use std::sync::LazyLock;

static V_GARAGE_FEATURE: LazyLock<VGarageFeature<crate::persistence::CachedVGarageRepository>> =
    LazyLock::new(|| {
        VGarageFeature::new(VGarageService::new(
            crate::persistence::v_garage_repository(),
        ))
    });

pub fn group() -> Group {
    Group::new()
        .command("init", init_garage)
        .command("get", get_garage)
        .command("save", save_garage)
        .command("delete", delete_garage)
}

pub(crate) fn init_garage(uid: String, unlocks_json: String) -> String {
    let unlocks = match serde_json::from_str::<VGarage>(&unlocks_json) {
        Ok(unlocks) => unlocks,
        Err(error) => {
            log::error(format_args!(
                "invalid virtual garage unlock payload: {error}"
            ));
            return format!("Error: invalid virtual garage unlock payload: {error}");
        }
    };

    match V_GARAGE_FEATURE.init(&uid, &unlocks) {
        Ok(garage) => response::json(&garage, "virtual garage"),
        Err(error) => {
            log::error(format_args!("failed to init virtual garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_garage(uid: String) -> String {
    match V_GARAGE_FEATURE.get(&uid) {
        Ok(Some(garage)) => response::json(&garage, "virtual garage"),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get virtual garage {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn save_garage(garage_json: String) -> String {
    let garage = match serde_json::from_str::<PlayerVGarage>(&garage_json) {
        Ok(garage) => garage,
        Err(error) => {
            log::error(format_args!("invalid virtual garage payload: {error}"));
            return format!("Error: invalid virtual garage payload: {error}");
        }
    };

    match V_GARAGE_FEATURE.save(garage) {
        Ok(garage) => response::json(&garage, "virtual garage"),
        Err(error) => {
            log::error(format_args!("failed to save virtual garage: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_garage(uid: String) -> String {
    match V_GARAGE_FEATURE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!(
                "failed to delete virtual garage {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}
