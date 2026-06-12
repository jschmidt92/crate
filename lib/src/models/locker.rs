use crate::shared::LockerError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Locker {
    #[serde(default)]
    pub items: HashMap<String, LockerItem>,
}

impl Locker {
    pub fn add_item(&mut self, item: LockerItem) -> Result<(), LockerError> {
        item.validate()?;
        self.items.insert(item.classname.clone(), item);
        Ok(())
    }

    pub fn remove_item(&mut self, classname: &str) -> Option<LockerItem> {
        self.items.remove(classname)
    }

    pub fn get_item(&self, classname: &str) -> Option<&LockerItem> {
        self.items.get(classname)
    }

    pub fn get_item_mut(&mut self, classname: &str) -> Option<&mut LockerItem> {
        self.items.get_mut(classname)
    }

    pub fn validate(&self) -> Result<(), LockerError> {
        for item in self.items.values() {
            item.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockerItem {
    pub category: String,
    pub classname: String,
    pub amount: u32,
}

impl LockerItem {
    pub fn new(
        category: impl Into<String>,
        classname: impl Into<String>,
        amount: u32,
    ) -> Result<Self, LockerError> {
        let item = Self {
            category: category.into(),
            classname: classname.into(),
            amount,
        };
        item.validate()?;
        Ok(item)
    }

    pub fn validate(&self) -> Result<(), LockerError> {
        if self.category.trim().is_empty() {
            return Err(LockerError::InvalidCategory);
        }

        if self.classname.trim().is_empty() {
            return Err(LockerError::InvalidClassname);
        }

        if self.amount == 0 {
            return Err(LockerError::InvalidAmount);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerLocker {
    pub uid: String,
    pub locker: Locker,
}

impl PlayerLocker {
    pub fn new(uid: impl Into<String>) -> Self {
        Self {
            uid: uid.into(),
            locker: Locker::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locker_adds_item_by_classname() {
        let mut locker = Locker::default();
        let item = LockerItem::new("weapons", "hgun_P07_F", 1).expect("item should be valid");

        locker.add_item(item).expect("item should be added");

        assert_eq!(
            locker.get_item("hgun_P07_F").map(|item| item.amount),
            Some(1)
        );
    }

    #[test]
    fn locker_rejects_zero_amount() {
        assert_eq!(
            LockerItem::new("items", "FirstAidKit", 0).expect_err("amount should be invalid"),
            LockerError::InvalidAmount
        );
    }
}
