use std::collection::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::spawn;
use tokio::sync::mpsc;

use crate::events::ForgeEvent;

type EventHandler =
    Box<dyn Fn(&ForgeEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct MessagingSystem {
    sender: mpsc::Sender<ForgeEvent>,
    handlers: HashMap<String, Vec<EventHandler>>,
}

impl MessagingSystem {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<ForgeEvent>(10240);
        let mut handlers: HashMap<String, Vec<EventHandler>> = HashMap::new();

        spawn(async move {
            while let Some(event) = rx.recv().await {
                let event_type = match &event {
                    ForgeEvent::Greet(_) => "Greet".to_string(),
                    ForgeEvent::Get(_) => "Get".to_string(),
                    ForgeEvent::Set { .. } => "Set".to_string(),
                    ForgeEvent::Del(_) => "Del".to_string(),
                    ForgeEvent::Exists(_) => "Exists".to_string(),
                    ForgeEvent::HGetAll(_) => "HGetAll".to_string(),
                    ForgeEvent::HGet { .. } => "HashGet".to_string(),
                    ForgeEvent::HSet { .. } => "HSet".to_string(),
                    ForgeEvent::HMGet { .. } => "HMGet".to_string(),
                    ForgeEvent::HMSet { .. } => "HMSet".to_string(),
                    ForgeEvent::HDel { .. } => "HDel".to_string(),
                    ForgeEvent::HExists { .. } => "HExists".to_string(),
                    ForgeEvent::HLen(_) => "HLen".to_string(),
                    ForgeEvent::HKeys(_) => "HKeys".to_string(),
                    ForgeEvent::HVals(_) => "HVals".to_string(),
                };

                if let Some(handlers) = handlers.get_mut(&event_type) {
                    for handler in handlers.iter_mut() {
                        handler(&event).await;
                    }
                }
            }
        });

        Self {
            sender: tx,
            handlers,
        }
    }
}

pub fn get_sender(&self) -> mpsc::Sender<ForgeEvent> {
    self.sender.clone()
}

pub fn register_handler<F, Fut>(&mut self, event_type: &str, handler: F)
where
    F: Fn(&ForgeEvent) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let boxed_handler: EventHandler = Box::new(move |event| Box::pin(handler(event)));
    self.handlers
        .entry(event_type.to_string())
        .or_insert_with(Vec::new)
        .push(boxed_handler);
}
