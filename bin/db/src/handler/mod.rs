mod events;
mod messaging;

use crate::events::ForgeEvent;
use crate::messaging::MessagingSystem;
use tokio::time::{self, Duration};
use tokio::{signal, spawn};

#[tokio::main]
async fn main() {
    let mut msq = MessagingSystem::new();

    msq.register_handler("Greet", |event| async move {
        if let ForgeEvent::Greet(name) = event {
            // TODO: implement
        }
    });
    msq.register_handler("HashGetAll", |event| async move {
        if let ForgeEvent::HGetAll(key) = event {
            // TODO: implement
        }
    });
    msq.register_handler("HashGet", |event| async move {
        if let ForgeEvent::HGet { key, field } = event {
            // TODO: implement
        }
    });

    let event_sender = msq.get_sender();

    println!("Press Ctrl+C to shut down");
    signal::ctrl_c().await?;
    println!("Shutdown signal received. Shutting down gracefully...");
    Ok(())
}
