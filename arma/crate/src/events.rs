use crate::{log, persistence};
use forge_lib::{
    events::{DomainEventHandler, EventBus, EventPublisher},
    models::DomainEvent,
    services::{BankService, GarageService, LockerService, VGarageService, VLockerService},
    shared::EventError,
};
use std::sync::LazyLock;

static EVENT_BUS: LazyLock<EventBus> = LazyLock::new(|| {
    EventBus::new()
        .subscribe(persistence::DurableEventBackend)
        .subscribe(ActorDisconnectHandler::new(
            "bank.player_disconnected",
            disconnect_bank,
        ))
        .subscribe(ActorDisconnectHandler::new(
            "garage.player_disconnected",
            disconnect_garage,
        ))
        .subscribe(ActorDisconnectHandler::new(
            "v_garage.player_disconnected",
            disconnect_v_garage,
        ))
        .subscribe(ActorDisconnectHandler::new(
            "locker.player_disconnected",
            disconnect_locker,
        ))
        .subscribe(ActorDisconnectHandler::new(
            "v_locker.player_disconnected",
            disconnect_v_locker,
        ))
});

type DisconnectFn = fn(&str) -> Result<(), String>;

struct ActorDisconnectHandler {
    name: &'static str,
    disconnect: DisconnectFn,
}

impl ActorDisconnectHandler {
    const fn new(name: &'static str, disconnect: DisconnectFn) -> Self {
        Self { name, disconnect }
    }
}

impl DomainEventHandler for ActorDisconnectHandler {
    fn name(&self) -> &'static str {
        self.name
    }

    fn handle(&self, event: &DomainEvent) -> Result<(), EventError> {
        let DomainEvent::ActorDisconnected(disconnected) = event else {
            return Ok(());
        };

        (self.disconnect)(&disconnected.uid).map_err(|message| EventError::HandlerFailed {
            handler: self.name(),
            event: event.name(),
            message,
        })
    }
}

fn disconnect_bank(uid: &str) -> Result<(), String> {
    BankService::new(persistence::bank_repository())
        .disconnect_player_account(uid)
        .map_err(|error| error.to_string())
}

fn disconnect_garage(uid: &str) -> Result<(), String> {
    GarageService::new(persistence::garage_repository())
        .disconnect(uid)
        .map_err(|error| error.to_string())
}

fn disconnect_v_garage(uid: &str) -> Result<(), String> {
    VGarageService::new(persistence::v_garage_repository())
        .disconnect(uid)
        .map_err(|error| error.to_string())
}

fn disconnect_locker(uid: &str) -> Result<(), String> {
    LockerService::new(persistence::locker_repository())
        .disconnect(uid)
        .map_err(|error| error.to_string())
}

fn disconnect_v_locker(uid: &str) -> Result<(), String> {
    VLockerService::new(persistence::v_locker_repository())
        .disconnect(uid)
        .map_err(|error| error.to_string())
}

pub(crate) fn init() {
    let _ = &*EVENT_BUS;
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ServerEventPublisher;

impl EventPublisher for ServerEventPublisher {
    fn publish(&self, event: DomainEvent) {
        publish(event);
    }

    fn publish_all(&self, events: &[DomainEvent]) {
        publish_all(events);
    }
}

pub(crate) fn publish(event: DomainEvent) {
    publish_all(&[event]);
}

pub(crate) fn publish_all(events: &[DomainEvent]) {
    for error in EVENT_BUS.publish_all(events) {
        log::error(format_args!("{error}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_lib::models::ActorDisconnected;

    fn succeeds(_: &str) -> Result<(), String> {
        Ok(())
    }

    fn fails(_: &str) -> Result<(), String> {
        Err("disconnect failed".to_string())
    }

    #[test]
    fn disconnect_handler_ignores_other_events() {
        let handler = ActorDisconnectHandler::new("test.disconnect", fails);
        let event = DomainEvent::ActorCreated(forge_lib::models::ActorCreated::new(
            forge_lib::models::Actor::from_snapshot(forge_lib::models::ActorSnapshot::new(
                "76561198000000000",
                "Tester",
            )),
            forge_lib::models::ActorStartingConfig::default(),
        ));

        assert!(handler.handle(&event).is_ok());
    }

    #[test]
    fn disconnect_handler_maps_feature_errors() {
        let handler = ActorDisconnectHandler::new("test.disconnect", fails);
        let event = DomainEvent::ActorDisconnected(ActorDisconnected::new("76561198000000000"));

        assert!(matches!(
            handler.handle(&event),
            Err(EventError::HandlerFailed {
                handler: "test.disconnect",
                event: "actor.disconnected",
                ..
            })
        ));
    }

    #[test]
    fn disconnect_handler_accepts_successful_cleanup() {
        let handler = ActorDisconnectHandler::new("test.disconnect", succeeds);
        let event = DomainEvent::ActorDisconnected(ActorDisconnected::new("76561198000000000"));

        assert!(handler.handle(&event).is_ok());
    }
}
