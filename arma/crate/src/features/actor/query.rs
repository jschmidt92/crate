use super::ActorFeature;
use forge_lib::{
    events::EventPublisher, models::Actor, repositories::ActorRepository, shared::ActorError,
};

impl<R, E> ActorFeature<R, E>
where
    R: ActorRepository,
    E: EventPublisher,
{
    pub(crate) fn get(&self, uid: &str) -> Result<Option<Actor>, ActorError> {
        self.service.get(uid)
    }
}
