use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::str::FromStr;
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

    pub fn to_amount(self) -> MoneyAmount {
        MoneyAmount::from(self)
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoneyAmount(String);

impl MoneyAmount {
    pub fn parse(&self) -> Option<Money> {
        Money::from_str(&self.0).ok()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Money> for MoneyAmount {
    fn from(value: Money) -> Self {
        Self(format!("{value}"))
    }
}

impl FromStr for Money {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if value.is_empty() {
            return Err(());
        }

        let amount = value.parse::<f64>().map_err(|_| ())?;
        Self::from_major(amount).ok_or(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBankProfile {
    pub uid: String,
    pub cash: Money,
    pub account: BankAccount,
    #[serde(default)]
    pub pending_earnings: Money,
    #[serde(default)]
    pub transactions: Vec<BankTransaction>,
    #[serde(default)]
    pin: Option<BankPin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBankProfileView {
    pub uid: String,
    pub cash: MoneyAmount,
    pub account: BankAccountView,
    pub pending_earnings: MoneyAmount,
    pub pin_set: bool,
    pub transactions: Vec<BankTransactionView>,
}

impl From<&PlayerBankProfile> for PlayerBankProfileView {
    fn from(profile: &PlayerBankProfile) -> Self {
        Self {
            uid: profile.uid.clone(),
            cash: profile.cash.to_amount(),
            account: BankAccountView::from(&profile.account),
            pending_earnings: profile.pending_earnings.to_amount(),
            pin_set: profile.pin.is_some(),
            transactions: profile
                .transactions
                .iter()
                .map(BankTransactionView::from)
                .collect(),
        }
    }
}

impl PlayerBankProfile {
    pub fn new(uid: impl Into<String>) -> Self {
        let uid = uid.into();
        let account = BankAccount::new(uid.clone());

        Self {
            uid,
            cash: Money::ZERO,
            account,
            pending_earnings: Money::ZERO,
            transactions: Vec::new(),
            pin: None,
        }
    }

    pub fn with_starting_balances(
        uid: impl Into<String>,
        cash: Money,
        bank_balance: Money,
    ) -> Self {
        let mut profile = Self::new(uid);
        profile.cash = cash;
        profile.account.deposit(bank_balance);
        profile
    }

    pub fn record_transaction(&mut self, amount: Money, description: impl Into<String>) {
        self.transactions.insert(
            0,
            BankTransaction::new(self.uid.clone(), amount, description),
        );
        self.transactions.truncate(10);
    }

    pub fn pin_is_set(&self) -> bool {
        self.pin.is_some()
    }

    pub fn verify_pin(&self, pin: &str) -> bool {
        self.pin
            .as_ref()
            .map(|stored| stored.verify(pin))
            .unwrap_or(false)
    }

    pub fn set_pin(&mut self, pin: &str) {
        self.pin = Some(BankPin::new(pin));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BankPin {
    salt: Uuid,
    hash: String,
}

impl BankPin {
    fn new(pin: &str) -> Self {
        let salt = Uuid::new_v4();
        Self {
            salt,
            hash: hash_pin(salt, pin),
        }
    }

    fn verify(&self, pin: &str) -> bool {
        self.hash == hash_pin(self.salt, pin)
    }
}

fn hash_pin(salt: Uuid, pin: &str) -> String {
    let digest = Sha256::digest(format!("{salt}:{pin}").as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub uid: String,
    pub balance: Money,
}

impl BankAccount {
    pub fn new(uid: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            uid: uid.into(),
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
pub struct BankAccountView {
    pub id: Uuid,
    pub uid: String,
    pub balance: MoneyAmount,
}

impl From<&BankAccount> for BankAccountView {
    fn from(account: &BankAccount) -> Self {
        Self {
            id: account.id,
            uid: account.uid.clone(),
            balance: account.balance.to_amount(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub id: Uuid,
    pub uid: String,
    pub amount: Money,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl BankTransaction {
    pub fn new(uid: impl Into<String>, amount: Money, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            uid: uid.into(),
            amount,
            description: description.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransactionView {
    pub id: Uuid,
    pub uid: String,
    pub amount: MoneyAmount,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl From<&BankTransaction> for BankTransactionView {
    fn from(transaction: &BankTransaction) -> Self {
        Self {
            id: transaction.id,
            uid: transaction.uid.clone(),
            amount: transaction.amount.to_amount(),
            description: transaction.description.clone(),
            created_at: transaction.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_amount_serializes_as_string() {
        let amount = Money::from_cents(1995).to_amount();

        assert_eq!(amount.as_str(), "19.95");
        assert_eq!(amount.parse(), Some(Money::from_cents(1995)));
    }

    #[test]
    fn bank_profile_view_exposes_money_as_string_amounts() {
        let mut profile = PlayerBankProfile::new("76561198000000000");
        profile.cash = Money::from_cents(1250);
        profile.account.deposit(Money::from_cents(2500));

        let view = PlayerBankProfileView::from(&profile);
        let json = serde_json::to_string(&view).expect("view should serialize");

        assert!(json.contains("\"cash\":\"12.50\""));
        assert!(json.contains("\"balance\":\"25.00\""));
        assert!(!json.contains("\"cents\""));
    }
}
