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

pub trait StorageError<T> {
    fn map_storage_error(self) -> Result<T, String>;
}

impl<T, E> StorageError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn map_storage_error(self) -> Result<T, String> {
        self.map_err(|error| error.to_string())
    }
}

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
pub enum VLockerError {
    InvalidUid,
    Repository(String),
}

impl std::fmt::Display for VLockerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUid => f.write_str("invalid virtual locker uid"),
            Self::Repository(error) => write!(f, "virtual locker repository error: {error}"),
        }
    }
}

impl std::error::Error for VLockerError {}

impl From<String> for VLockerError {
    fn from(value: String) -> Self {
        Self::Repository(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VGarageError {
    InvalidUid,
    Repository(String),
}

impl std::fmt::Display for VGarageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUid => f.write_str("invalid virtual garage uid"),
            Self::Repository(error) => write!(f, "virtual garage repository error: {error}"),
        }
    }
}

impl std::error::Error for VGarageError {}

impl From<String> for VGarageError {
    fn from(value: String) -> Self {
        Self::Repository(value)
    }
}

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
