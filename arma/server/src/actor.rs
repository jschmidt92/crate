use crate::log;
use arma_rs::Group;
use forge_lib::{
    events::{EventBus, handlers::BankActorCreatedHandler},
    models::ActorSnapshot,
    repositories::InMemoryActorRepository,
    services::ActorService,
};
use std::sync::LazyLock;

static ACTOR_SERVICE: LazyLock<ActorService<InMemoryActorRepository>> =
    LazyLock::new(|| ActorService::new(InMemoryActorRepository::new()));

static EVENT_BUS: LazyLock<EventBus> =
    LazyLock::new(|| EventBus::new().subscribe(BankActorCreatedHandler));

pub fn group() -> Group {
    Group::new()
        .command("init", init_actor)
        .command("get", get_actor)
        .command("delete", delete_actor)
}

fn init_actor(snapshot_json: String) -> String {
    let snapshot = match serde_json::from_str::<ActorSnapshot>(&snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            log::error(format_args!("invalid actor init snapshot: {error}"));
            return format!("Error: invalid actor init snapshot: {error}");
        }
    };

    match ACTOR_SERVICE.init_or_create(snapshot) {
        Ok(result) => {
            let errors = EVENT_BUS.publish_all(&result.events);
            for error in errors {
                log::error(format_args!("{error}"));
            }

            match serde_json::to_string(&result.actor) {
                Ok(json) => json,
                Err(error) => {
                    log::error(format_args!(
                        "failed to serialize actor init result: {error}"
                    ));
                    format!("Error: failed to serialize actor init result: {error}")
                }
            }
        }
        Err(error) => {
            log::error(format_args!("failed to init actor: {error}"));
            format!("Error: {error}")
        }
    }
}

fn get_actor(uid: String) -> String {
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

fn delete_actor(uid: String) -> String {
    match ACTOR_SERVICE.delete(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!("failed to delete actor {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
