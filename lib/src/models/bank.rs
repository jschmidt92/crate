use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.as_major())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBankProfile {
    pub player_id: u64,
    pub cash: CashWallet,
    pub accounts: Vec<BankAccount>,
}

impl PlayerBankProfile {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            cash: CashWallet::default(),
            accounts: vec![BankAccount::personal(player_id)],
        }
    }

    pub fn primary_account(&self) -> Option<&BankAccount> {
        self.accounts
            .iter()
            .find(|account| account.kind == AccountKind::Personal)
    }

    pub fn primary_account_mut(&mut self) -> Option<&mut BankAccount> {
        self.accounts
            .iter_mut()
            .find(|account| account.kind == AccountKind::Personal)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CashWallet {
    pub balance: Money,
}

impl CashWallet {
    pub fn deposit(&mut self, amount: Money) {
        self.balance = self.balance + amount;
    }

    pub fn withdraw(&mut self, amount: Money) -> bool {
        if self.balance < amount {
            return false;
        }

        self.balance = self.balance - amount;
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub owner_player_id: u64,
    pub kind: AccountKind,
    pub status: AccountStatus,
    pub balance: Money,
}

impl BankAccount {
    pub fn personal(owner_player_id: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_player_id,
            kind: AccountKind::Personal,
            status: AccountStatus::Open,
            balance: Money::ZERO,
        }
    }

    pub fn can_debit(&self, amount: Money) -> bool {
        self.status == AccountStatus::Open && amount.is_positive() && self.balance >= amount
    }

    pub fn credit(&mut self, amount: Money) {
        self.balance = self.balance + amount;
    }

    pub fn debit(&mut self, amount: Money) -> bool {
        if !self.can_debit(amount) {
            return false;
        }

        self.balance = self.balance - amount;
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    Personal,
    Business,
    Government,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    Open,
    Frozen,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub id: Uuid,
    pub player_id: u64,
    pub account_id: Option<Uuid>,
    pub amount: Money,
    pub kind: TransactionKind,
    pub direction: TransactionDirection,
    pub status: TransactionStatus,
    pub description: String,
    pub reference: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl BankTransaction {
    pub fn completed(
        player_id: u64,
        account_id: Option<Uuid>,
        amount: Money,
        kind: TransactionKind,
        direction: TransactionDirection,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            account_id,
            amount,
            kind,
            direction,
            status: TransactionStatus::Completed,
            description: description.into(),
            reference: None,
            created_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Transfer,
    Purchase,
    FuelPurchase,
    Adjustment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionDirection {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

impl Default for Money {
    fn default() -> Self {
        Self::ZERO
    }
}
