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
    pub(crate) fn disconnect(&self, snapshot: ActorSnapshot) -> Result<Actor, ActorError> {
        self.service.disconnect(snapshot)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), ActorError> {
        self.service.delete(uid)
    }
}
