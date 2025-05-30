mod actor;
mod events;
mod messaging;

use bin::db::src::handler::actor::register_actor_handlers;

use crate::actor::Actor;
use crate::events::ForgeEvent;
use crate::messaging::MessagingSystem;

use std::error::Error;
use tokio::time::{self, Duration};
use tokio::{signal, spawn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut msq = MessagingSystem::new();

    register_actor_handlers(&mut msq);

    let event_sender = msq.get_sender();

    println!("Press Ctrl+C to shut down");
    signal::ctrl_c().await?;
    println!("Shutdown signal received. Shutting down gracefully...");
    Ok(())
}
