use crate::log;
use arma_rs::Group;
use forge_lib::{models::ActorSnapshot, services::ActorService};
use std::sync::LazyLock;

static ACTOR_SERVICE: LazyLock<ActorService<crate::persistence::CachedActorRepository>> =
    LazyLock::new(|| ActorService::new(crate::persistence::actor_repository()));

pub fn group() -> Group {
    Group::new()
        .command("init", init_actor)
        .command("disconnect", disconnect_actor)
        .command("disconnect_uid", disconnect_actor_uid)
        .command("get", get_actor)
        .command("delete", delete_actor)
}

pub(crate) fn init_actor(snapshot_json: String) -> String {
    let snapshot = match serde_json::from_str::<ActorSnapshot>(&snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            log::error(format_args!("invalid actor init snapshot: {error}"));
            return format!("Error: invalid actor init snapshot: {error}");
        }
    };

    match ACTOR_SERVICE.init_or_create(snapshot) {
        Ok(result) => match serde_json::to_string(&result.actor) {
            Ok(json) => json,
            Err(error) => {
                log::error(format_args!(
                    "failed to serialize actor init result: {error}"
                ));
                format!("Error: failed to serialize actor init result: {error}")
            }
        },
        Err(error) => {
            log::error(format_args!("failed to init actor: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_actor(snapshot_json: String) -> String {
    let snapshot = match serde_json::from_str::<ActorSnapshot>(&snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            log::error(format_args!("invalid actor disconnect snapshot: {error}"));
            return format!("Error: invalid actor disconnect snapshot: {error}");
        }
    };

    match ACTOR_SERVICE.disconnect(snapshot) {
        Ok(actor) => match serde_json::to_string(&actor) {
            Ok(json) => json,
            Err(error) => {
                log::error(format_args!(
                    "failed to serialize actor disconnect result: {error}"
                ));
                format!("Error: failed to serialize actor disconnect result: {error}")
            }
        },
        Err(error) => {
            log::error(format_args!("failed to disconnect actor: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_actor_uid(uid: String) -> String {
    match ACTOR_SERVICE.get(&uid) {
        Ok(_) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to disconnect actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_actor(uid: String) -> String {
    match ACTOR_SERVICE.get(&uid) {
        Ok(Some(actor)) => match serde_json::to_string(&actor) {
            Ok(json) => json,
            Err(error) => {
                log::error(format_args!("failed to serialize actor: {error}"));
                format!("Error: failed to serialize actor: {error}")
            }
        },
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_actor(uid: String) -> String {
    match ACTOR_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
