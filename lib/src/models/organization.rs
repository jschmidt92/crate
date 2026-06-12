use serde::{Deserialize, Serialize};

use super::{Money, MoneyAmount, VGarage, VLocker};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub kind: OrganizationKind,
    pub members: Vec<OrganizationMember>,
    pub bank: Money,
    pub virtual_garage: VGarage,
    pub virtual_locker: VLocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationView {
    pub id: String,
    pub name: String,
    pub kind: OrganizationKind,
    pub members: Vec<OrganizationMember>,
    pub bank: MoneyAmount,
    pub virtual_garage: VGarage,
    pub virtual_locker: VLocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPayday {
    pub organization: OrganizationView,
    pub amount: MoneyAmount,
    pub recipients: Vec<String>,
}

impl From<&Organization> for OrganizationView {
    fn from(organization: &Organization) -> Self {
        Self {
            id: organization.id.clone(),
            name: organization.name.clone(),
            kind: organization.kind,
            members: organization.members.clone(),
            bank: organization.bank.to_amount(),
            virtual_garage: organization.virtual_garage.clone(),
            virtual_locker: organization.virtual_locker.clone(),
        }
    }
}

impl Organization {
    pub fn default_org() -> Self {
        Self::default_org_with_starting(Money::ZERO, VGarage::default(), VLocker::default())
    }

    pub fn default_org_with_starting(
        bank: Money,
        virtual_garage: VGarage,
        virtual_locker: VLocker,
    ) -> Self {
        Self {
            id: "default".to_string(),
            name: "Default".to_string(),
            kind: OrganizationKind::Default,
            members: Vec::new(),
            bank,
            virtual_garage,
            virtual_locker,
        }
    }

    pub fn player_org(
        id: impl Into<String>,
        name: impl Into<String>,
        ceo_uid: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: OrganizationKind::Player,
            members: vec![OrganizationMember::new(ceo_uid, OrganizationRole::Ceo)],
            bank: Money::ZERO,
            virtual_garage: VGarage::default(),
            virtual_locker: VLocker::default(),
        }
    }

    pub fn member(&self, uid: &str) -> Option<&OrganizationMember> {
        self.members.iter().find(|member| member.uid == uid)
    }

    pub fn has_member(&self, uid: &str) -> bool {
        self.member(uid).is_some()
    }

    pub fn is_ceo(&self, uid: &str) -> bool {
        self.member(uid)
            .map(|member| member.role == OrganizationRole::Ceo)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationKind {
    Default,
    Player,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub uid: String,
    pub role: OrganizationRole,
}

impl OrganizationMember {
    pub fn new(uid: impl Into<String>, role: OrganizationRole) -> Self {
        Self {
            uid: uid.into(),
            role,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationRole {
    Ceo,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationAction {
    Rename,
    Disband,
    InviteMember,
    RemoveMember,
    SpendFunds,
    BuyOrgUnlock,
    IssuePayday,
}

impl OrganizationAction {
    pub const fn requires_admin_policy(self) -> bool {
        matches!(
            self,
            Self::Rename | Self::Disband | Self::InviteMember | Self::RemoveMember
        )
    }
}
