use crate::{log, persistence};
use forge_lib::{events::EventBus, models::DomainEvent};
use std::sync::LazyLock;

static EVENT_BUS: LazyLock<EventBus> =
    LazyLock::new(|| EventBus::new().subscribe(persistence::DurableEventBackend));

pub(crate) fn init() {
    let _ = &*EVENT_BUS;
}

pub(crate) fn publish(event: DomainEvent) {
    publish_all(&[event]);
}

pub(crate) fn publish_all(events: &[DomainEvent]) {
    for error in EVENT_BUS.publish_all(events) {
        log::error(format_args!("{error}"));
    }
}
