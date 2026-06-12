pub mod actor;
pub mod v_garage;
pub mod v_locker;

pub use actor::{ActorRepository, InMemoryActorRepository};
pub use v_garage::{InMemoryVGarageRepository, VGarageRepository};
pub use v_locker::{InMemoryVLockerRepository, VLockerRepository};
