use crate::{events::ServerEventPublisher, features::actor::ActorFeature, log, response};
use arma_rs::Group;
use forge_lib::{models::ActorSnapshot, services::ActorService};
use std::sync::LazyLock;

static ACTOR_FEATURE: LazyLock<
    ActorFeature<crate::persistence::CachedActorRepository, ServerEventPublisher>,
> = LazyLock::new(|| {
    ActorFeature::new(
        ActorService::new(crate::persistence::actor_repository()),
        ServerEventPublisher,
    )
});

pub fn group() -> Group {
    Group::new()
        .command("init", init_actor)
        .command("save", save_actor)
        .command("disconnect", disconnect_actor)
        .command("get", get_actor)
        .command("delete", delete_actor)
}

pub(crate) fn save_actor(snapshot_json: String) -> String {
    let snapshot = match serde_json::from_str::<ActorSnapshot>(&snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            log::error(format_args!("invalid actor save snapshot: {error}"));
            return format!("Error: invalid actor save snapshot: {error}");
        }
    };

    match ACTOR_FEATURE.save_snapshot(snapshot) {
        Ok(actor) => response::json(&actor, "actor save result"),
        Err(error) => {
            log::error(format_args!("failed to save actor: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn init_actor(snapshot_json: String) -> String {
    let snapshot = match serde_json::from_str::<ActorSnapshot>(&snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            log::error(format_args!("invalid actor init snapshot: {error}"));
            return format!("Error: invalid actor init snapshot: {error}");
        }
    };

    match ACTOR_FEATURE.init_or_create(snapshot) {
        Ok(result) => {
            log::debug(format_args!(
                "actor initialization completed uid={} created={}",
                result.actor.uid, result.created
            ));
            response::json(
                &serde_json::json!({
                    "actor": result.actor,
                    "created": result.created,
                }),
                "actor init result",
            )
        }
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

    match ACTOR_FEATURE.disconnect(snapshot) {
        Ok(actor) => response::json(&actor, "actor disconnect result"),
        Err(error) => {
            log::error(format_args!("failed to disconnect actor: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_actor(uid: String) -> String {
    match ACTOR_FEATURE.get(&uid) {
        Ok(Some(actor)) => response::json(&actor, "actor"),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn delete_actor(uid: String) -> String {
    match ACTOR_FEATURE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
