#[derive(Debug, Clone)]
pub enum BankError {
    InvalidAmount,
    InvalidActorUid,
}

impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAmount => f.write_str("invalid transaction amount"),
            Self::InvalidActorUid => f.write_str("invalid actor uid"),
        }
    }
}

impl std::error::Error for BankError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorError {
    InvalidUid,
    InvalidName,
    Repository(String),
}

impl std::fmt::Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUid => f.write_str("invalid actor uid"),
            Self::InvalidName => f.write_str("invalid actor name"),
            Self::Repository(error) => write!(f, "actor repository error: {error}"),
        }
    }
}

impl std::error::Error for ActorError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventError {
    HandlerFailed {
        handler: &'static str,
        event: &'static str,
        message: String,
    },
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HandlerFailed {
                handler,
                event,
                message,
            } => write!(f, "event handler {handler} failed for {event}: {message}"),
        }
    }
}

impl std::error::Error for EventError {}
