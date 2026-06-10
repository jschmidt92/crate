#[derive(Debug, Clone)]
pub enum BankError {
    InvalidAmount,
}

impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAmount => f.write_str("invalid transaction amount"),
        }
    }
}

impl std::error::Error for BankError {}
