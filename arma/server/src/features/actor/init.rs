use super::ActorFeature;
use forge_lib::{
    events::EventPublisher,
    models::{Actor, ActorSnapshot},
    repositories::ActorRepository,
    shared::ActorError,
};

impl<R, E> ActorFeature<R, E>
where
    R: ActorRepository,
    E: EventPublisher,
{
    pub(crate) fn init_or_create(&self, snapshot: ActorSnapshot) -> Result<Actor, ActorError> {
        let result = self.service.init_or_create(snapshot)?;
        self.events.publish_all(&result.events);
        Ok(result.actor)
    }
}
