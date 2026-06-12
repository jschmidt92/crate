use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    cents: i64,
}

impl Money {
    pub const ZERO: Self = Self { cents: 0 };

    pub const fn from_cents(cents: i64) -> Self {
        Self { cents }
    }

    pub fn from_major(amount: f64) -> Option<Self> {
        if !amount.is_finite() {
            return None;
        }

        Some(Self {
            cents: (amount * 100.0).round() as i64,
        })
    }

    pub const fn cents(self) -> i64 {
        self.cents
    }

    pub fn as_major(self) -> f64 {
        self.cents as f64 / 100.0
    }

    pub const fn is_positive(self) -> bool {
        self.cents > 0
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.as_major())
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_cents(self.cents + rhs.cents)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_cents(self.cents - rhs.cents)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBankProfile {
    pub player_id: u64,
    pub account: BankAccount,
}

impl PlayerBankProfile {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            account: BankAccount::new(player_id),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub player_id: u64,
    pub balance: Money,
}

impl BankAccount {
    pub fn new(player_id: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            balance: Money::ZERO,
        }
    }

    pub fn deposit(&mut self, amount: Money) {
        self.balance = self.balance + amount;
    }

    pub fn withdraw(&mut self, amount: Money) -> bool {
        if !amount.is_positive() || self.balance < amount {
            return false;
        }

        self.balance = self.balance - amount;
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub id: Uuid,
    pub player_id: u64,
    pub amount: Money,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl BankTransaction {
    pub fn new(player_id: u64, amount: Money, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            amount,
            description: description.into(),
            created_at: Utc::now(),
        }
    }
}
