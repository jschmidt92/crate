mod init;
mod lifecycle;
mod query;

use forge_lib::{events::EventPublisher, repositories::ActorRepository, services::ActorService};

#[derive(Clone)]
pub(crate) struct ActorFeature<R, E> {
    service: ActorService<R>,
    events: E,
}

impl<R, E> ActorFeature<R, E>
where
    R: ActorRepository,
    E: EventPublisher,
{
    pub(crate) const fn new(service: ActorService<R>, events: E) -> Self {
        Self { service, events }
    }
}
