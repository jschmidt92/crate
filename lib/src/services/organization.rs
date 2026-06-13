use crate::{
    models::{
        Money, Organization, OrganizationAction, OrganizationDisband, OrganizationInvite,
        OrganizationInviteStatus, OrganizationKind, OrganizationMember, OrganizationMemberTransfer,
        OrganizationPayday, OrganizationPaydayPlan, OrganizationRole, OrganizationView, VGarage,
        VLocker,
    },
    repositories::OrganizationRepository,
    shared::OrganizationError,
};

#[derive(Clone)]
pub struct OrganizationService<R> {
    repository: R,
}

impl<R> OrganizationService<R>
where
    R: OrganizationRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_default_org(&self) -> Result<Organization, OrganizationError> {
        self.create_default_org_with_starting("0.00", &VGarage::default(), &VLocker::default())
    }

    pub fn create_default_org_with_starting(
        &self,
        starting_bank: &str,
        virtual_garage: &VGarage,
        virtual_locker: &VLocker,
    ) -> Result<Organization, OrganizationError> {
        if let Some(organization) = self.repository.find_by_id("default")? {
            return Ok(organization);
        }

        let bank = parse_starting_money(starting_bank)?;
        self.repository
            .save(Organization::default_org_with_starting(
                bank,
                virtual_garage.clone(),
                virtual_locker.clone(),
            ))
    }

    pub fn create_player_org(
        &self,
        id: &str,
        name: &str,
        ceo_uid: &str,
    ) -> Result<Organization, OrganizationError> {
        validate_org_id(id)?;
        validate_uid(ceo_uid)?;

        self.remove_from_current_org_for_transfer(ceo_uid)?;
        self.repository
            .save(Organization::player_org(id, name, ceo_uid))
    }

    pub fn create_invite(
        &self,
        inviter_uid: &str,
        organization_id: &str,
        invitee_uid: &str,
    ) -> Result<OrganizationInvite, OrganizationError> {
        let organization = self.require_action_allowed(
            inviter_uid,
            organization_id,
            OrganizationAction::InviteMember,
        )?;
        validate_uid(invitee_uid)?;

        if organization.has_member(invitee_uid) {
            return Err(OrganizationError::AlreadyMember);
        }

        if self
            .repository
            .find_pending_invite(organization_id, invitee_uid)?
            .is_some()
        {
            return Err(OrganizationError::InviteNotPending);
        }

        self.repository.save_invite(OrganizationInvite::new(
            organization_id,
            inviter_uid,
            invitee_uid,
        ))
    }

    pub fn accept_invite(
        &self,
        invitee_uid: &str,
        invite_id: &str,
    ) -> Result<(Organization, OrganizationInvite), OrganizationError> {
        validate_uid(invitee_uid)?;
        let mut invite = self.require_invite(invite_id)?;
        self.require_pending_invite_for(&invite, invitee_uid)?;

        let mut organization = self.require_org(&invite.organization_id)?;
        if organization.has_member(invitee_uid) {
            return Err(OrganizationError::AlreadyMember);
        }

        self.remove_from_current_org_for_transfer(invitee_uid)?;
        if !organization.has_member(invitee_uid) {
            organization.members.push(OrganizationMember::new(
                invitee_uid,
                OrganizationRole::Member,
            ));
        }
        invite.accept();

        let organization = self.repository.save(organization)?;
        let invite = self.repository.save_invite(invite)?;

        Ok((organization, invite))
    }

    pub fn decline_invite(
        &self,
        invitee_uid: &str,
        invite_id: &str,
    ) -> Result<OrganizationInvite, OrganizationError> {
        validate_uid(invitee_uid)?;
        let mut invite = self.require_invite(invite_id)?;
        self.require_pending_invite_for(&invite, invitee_uid)?;
        invite.decline();

        self.repository.save_invite(invite)
    }

    pub fn leave_member(
        &self,
        organization_id: &str,
        uid: &str,
    ) -> Result<OrganizationMemberTransfer, OrganizationError> {
        let mut organization = self.require_member(uid, organization_id)?;

        if organization.kind == OrganizationKind::Default {
            return Err(OrganizationError::CannotLeaveDefaultOrg);
        }

        if organization.is_ceo(uid) {
            return Err(OrganizationError::LastCeoCannotLeave);
        }

        organization.members.retain(|member| member.uid != uid);
        let organization = self.repository.save(organization)?;
        let default_organization = self.add_to_default_org(uid)?;

        Ok(OrganizationMemberTransfer {
            organization: OrganizationView::from(&organization),
            default_organization: OrganizationView::from(&default_organization),
            uid: uid.to_string(),
        })
    }

    pub fn kick_member(
        &self,
        organization_id: &str,
        actor_uid: &str,
        kicked_uid: &str,
    ) -> Result<OrganizationMemberTransfer, OrganizationError> {
        let organization = self.require_org(organization_id)?;
        if organization.kind == OrganizationKind::Default {
            return Err(OrganizationError::RestrictedDefaultOrgAction);
        }

        let mut organization = self.require_action_allowed(
            actor_uid,
            organization_id,
            OrganizationAction::RemoveMember,
        )?;
        validate_uid(kicked_uid)?;

        if !organization.has_member(kicked_uid) {
            return Err(OrganizationError::NotMember);
        }

        if organization.is_ceo(kicked_uid) {
            return Err(OrganizationError::LastCeoCannotLeave);
        }

        organization
            .members
            .retain(|member| member.uid != kicked_uid);
        let organization = self.repository.save(organization)?;
        let default_organization = self.add_to_default_org(kicked_uid)?;

        Ok(OrganizationMemberTransfer {
            organization: OrganizationView::from(&organization),
            default_organization: OrganizationView::from(&default_organization),
            uid: kicked_uid.to_string(),
        })
    }

    pub fn disband_player_org(
        &self,
        organization_id: &str,
        ceo_uid: &str,
    ) -> Result<OrganizationDisband, OrganizationError> {
        let organization =
            self.require_action_allowed(ceo_uid, organization_id, OrganizationAction::Disband)?;

        if organization.kind != OrganizationKind::Player {
            return Err(OrganizationError::RestrictedDefaultOrgAction);
        }

        let mut default_organization = self
            .repository
            .find_by_id("default")?
            .unwrap_or_else(Organization::default_org);

        let mut reassigned_uids = Vec::new();
        for uid in organization.members.iter().map(|member| member.uid.clone()) {
            validate_uid(&uid)?;
            if !default_organization.has_member(&uid) {
                default_organization
                    .members
                    .push(OrganizationMember::new(&uid, OrganizationRole::Member));
            }
            if !reassigned_uids.contains(&uid) {
                reassigned_uids.push(uid);
            }
        }

        let default_organization = self.repository.save(default_organization)?;
        self.repository.delete(&organization.id)?;

        Ok(OrganizationDisband {
            disbanded: OrganizationView::from(&organization),
            default_organization: OrganizationView::from(&default_organization),
            reassigned_uids,
        })
    }

    pub fn get(&self, id: &str) -> Result<Option<Organization>, OrganizationError> {
        validate_org_id(id)?;
        self.repository.find_by_id(id)
    }

    pub fn get_by_member_uid(&self, uid: &str) -> Result<Option<Organization>, OrganizationError> {
        validate_uid(uid)?;
        self.repository.find_by_member_uid(uid)
    }

    pub fn add_member(
        &self,
        organization_id: &str,
        uid: &str,
    ) -> Result<Organization, OrganizationError> {
        let mut organization = self.require_org(organization_id)?;
        validate_uid(uid)?;

        if !organization.has_member(uid) {
            organization
                .members
                .push(crate::models::OrganizationMember::new(
                    uid,
                    crate::models::OrganizationRole::Member,
                ));
        }

        self.repository.save(organization)
    }

    pub fn require_member(
        &self,
        uid: &str,
        organization_id: &str,
    ) -> Result<Organization, OrganizationError> {
        let organization = self.require_org(organization_id)?;
        validate_uid(uid)?;

        if !organization.has_member(uid) {
            return Err(OrganizationError::NotMember);
        }

        Ok(organization)
    }

    pub fn require_ceo(
        &self,
        uid: &str,
        organization_id: &str,
    ) -> Result<Organization, OrganizationError> {
        let organization = self.require_member(uid, organization_id)?;

        if !organization.is_ceo(uid) {
            return Err(OrganizationError::NotCeo);
        }

        Ok(organization)
    }

    pub fn require_action_allowed(
        &self,
        uid: &str,
        organization_id: &str,
        action: OrganizationAction,
    ) -> Result<Organization, OrganizationError> {
        let organization = self.require_ceo(uid, organization_id)?;

        if organization.kind == OrganizationKind::Default && action.requires_admin_policy() {
            return Err(OrganizationError::RestrictedDefaultOrgAction);
        }

        Ok(organization)
    }

    pub fn require_action_allowed_with_default_ceo_slot(
        &self,
        uid: &str,
        organization_id: &str,
        action: OrganizationAction,
        in_default_ceo_slot: bool,
    ) -> Result<Organization, OrganizationError> {
        let organization = self.require_org(organization_id)?;
        validate_uid(uid)?;

        if organization.kind != OrganizationKind::Default {
            return self.require_action_allowed(uid, organization_id, action);
        }

        if !in_default_ceo_slot {
            return Err(OrganizationError::NotCeo);
        }

        if action.requires_admin_policy() {
            return Err(OrganizationError::RestrictedDefaultOrgAction);
        }

        Ok(organization)
    }

    pub fn require_spend_allowed(
        &self,
        uid: &str,
        organization_id: &str,
    ) -> Result<Organization, OrganizationError> {
        self.require_action_allowed(uid, organization_id, OrganizationAction::SpendFunds)
    }

    pub fn require_buy_unlock_allowed(
        &self,
        uid: &str,
        organization_id: &str,
    ) -> Result<Organization, OrganizationError> {
        self.require_action_allowed(uid, organization_id, OrganizationAction::BuyOrgUnlock)
    }

    pub fn require_spend_allowed_with_default_ceo_slot(
        &self,
        uid: &str,
        organization_id: &str,
        in_default_ceo_slot: bool,
    ) -> Result<Organization, OrganizationError> {
        self.require_action_allowed_with_default_ceo_slot(
            uid,
            organization_id,
            OrganizationAction::SpendFunds,
            in_default_ceo_slot,
        )
    }

    pub fn require_buy_unlock_allowed_with_default_ceo_slot(
        &self,
        uid: &str,
        organization_id: &str,
        in_default_ceo_slot: bool,
    ) -> Result<Organization, OrganizationError> {
        self.require_action_allowed_with_default_ceo_slot(
            uid,
            organization_id,
            OrganizationAction::BuyOrgUnlock,
            in_default_ceo_slot,
        )
    }

    pub fn issue_payday(
        &self,
        uid: &str,
        organization_id: &str,
        amount: &str,
        in_default_ceo_slot: bool,
    ) -> Result<OrganizationPayday, OrganizationError> {
        let plan = self.prepare_payday(uid, organization_id, amount, in_default_ceo_slot)?;
        let organization = self.repository.save(plan.organization)?;

        Ok(OrganizationPayday {
            organization: OrganizationView::from(&organization),
            amount: plan.amount.to_amount(),
            recipients: plan.recipients,
        })
    }

    pub fn prepare_payday(
        &self,
        uid: &str,
        organization_id: &str,
        amount: &str,
        in_default_ceo_slot: bool,
    ) -> Result<OrganizationPaydayPlan, OrganizationError> {
        let mut organization = self.require_action_allowed_with_default_ceo_slot(
            uid,
            organization_id,
            OrganizationAction::IssuePayday,
            in_default_ceo_slot,
        )?;
        let amount = parse_starting_money(amount)?;
        if !amount.is_positive() {
            return Err(OrganizationError::InvalidAmount);
        }

        let mut recipients = Vec::new();
        for recipient_uid in organization
            .members
            .iter()
            .map(|member| member.uid.clone())
            .filter(|member_uid| member_uid != uid)
        {
            validate_uid(&recipient_uid)?;
            if !recipients.contains(&recipient_uid) {
                recipients.push(recipient_uid);
            }
        }

        if recipients.is_empty() {
            return Err(OrganizationError::EmptyPayday);
        }

        let total = Money::from_cents(amount.cents() * recipients.len() as i64);
        if organization.bank < total {
            return Err(OrganizationError::InsufficientFunds);
        }

        organization.bank = organization.bank - total;

        Ok(OrganizationPaydayPlan {
            organization,
            amount,
            recipients,
        })
    }

    fn require_org(&self, organization_id: &str) -> Result<Organization, OrganizationError> {
        validate_org_id(organization_id)?;
        self.repository
            .find_by_id(organization_id)?
            .ok_or(OrganizationError::NotFound)
    }

    fn require_invite(&self, invite_id: &str) -> Result<OrganizationInvite, OrganizationError> {
        validate_org_id(invite_id)?;
        self.repository
            .find_invite(invite_id)?
            .ok_or(OrganizationError::InviteNotFound)
    }

    fn require_pending_invite_for(
        &self,
        invite: &OrganizationInvite,
        uid: &str,
    ) -> Result<(), OrganizationError> {
        if invite.invitee_uid != uid {
            return Err(OrganizationError::InviteeMismatch);
        }

        if invite.status != OrganizationInviteStatus::Pending {
            return Err(OrganizationError::InviteNotPending);
        }

        Ok(())
    }

    fn remove_from_current_org_for_transfer(&self, uid: &str) -> Result<(), OrganizationError> {
        let Some(mut current_org) = self.repository.find_by_member_uid(uid)? else {
            return Ok(());
        };

        if current_org.is_ceo(uid) && current_org.kind == OrganizationKind::Player {
            return Err(OrganizationError::LastCeoCannotLeave);
        }

        current_org.members.retain(|member| member.uid != uid);
        self.repository.save(current_org)?;
        Ok(())
    }

    fn add_to_default_org(&self, uid: &str) -> Result<Organization, OrganizationError> {
        validate_uid(uid)?;
        let mut default_organization = self
            .repository
            .find_by_id("default")?
            .unwrap_or_else(Organization::default_org);

        if !default_organization.has_member(uid) {
            default_organization
                .members
                .push(OrganizationMember::new(uid, OrganizationRole::Member));
        }

        self.repository.save(default_organization)
    }
}

fn validate_org_id(id: &str) -> Result<(), OrganizationError> {
    if id.trim().is_empty() {
        return Err(OrganizationError::InvalidId);
    }

    Ok(())
}

fn validate_uid(uid: &str) -> Result<(), OrganizationError> {
    if uid.trim().is_empty() {
        return Err(OrganizationError::InvalidUid);
    }

    Ok(())
}

fn parse_starting_money(amount: &str) -> Result<Money, OrganizationError> {
    let money = amount
        .parse::<Money>()
        .map_err(|_| OrganizationError::InvalidAmount)?;

    if money.cents() < 0 {
        return Err(OrganizationError::InvalidAmount);
    }

    Ok(money)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{OrganizationAction, OrganizationMember, OrganizationRole},
        repositories::InMemoryOrganizationRepository,
    };

    #[test]
    fn default_org_has_no_player_uid_ceo_member() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        let organization = service
            .create_default_org()
            .expect("default org should be created");

        assert!(organization.members.is_empty());
        assert!(!organization.is_ceo("ceo-uid"));
        assert_eq!(organization.bank, Money::ZERO);
    }

    #[test]
    fn add_member_adds_default_org_member_once() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");

        service
            .add_member("default", "member-uid")
            .expect("member should be added");
        let organization = service
            .add_member("default", "member-uid")
            .expect("member add should be idempotent");

        assert_eq!(organization.members.len(), 1);
        assert!(organization.has_member("member-uid"));
        assert!(!organization.is_ceo("member-uid"));
    }

    #[test]
    fn default_org_uses_configured_starting_values() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        let garage = VGarage {
            cars: vec!["B_Quadbike_01_F".to_string()],
            ..VGarage::default()
        };
        let locker = VLocker {
            weapons: vec!["hgun_P07_F".to_string()],
            ..VLocker::default()
        };

        let organization = service
            .create_default_org_with_starting("1000.50", &garage, &locker)
            .expect("default org should be created");

        assert_eq!(organization.bank.to_amount().as_str(), "1000.50");
        assert_eq!(organization.virtual_garage.cars, ["B_Quadbike_01_F"]);
        assert_eq!(organization.virtual_locker.weapons, ["hgun_P07_F"]);
    }

    #[test]
    fn default_org_ceo_slot_can_spend_and_buy_unlocks() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");

        assert!(
            service
                .require_spend_allowed_with_default_ceo_slot("ceo-uid", "default", true)
                .is_ok()
        );
        assert!(
            service
                .require_buy_unlock_allowed_with_default_ceo_slot("ceo-uid", "default", true)
                .is_ok()
        );
    }

    #[test]
    fn default_org_ceo_cannot_perform_restricted_admin_actions() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");

        let error = service
            .require_action_allowed_with_default_ceo_slot(
                "ceo-uid",
                "default",
                OrganizationAction::Rename,
                true,
            )
            .expect_err("default org rename should be blocked");

        assert_eq!(error, OrganizationError::RestrictedDefaultOrgAction);
    }

    #[test]
    fn default_org_non_slot_player_cannot_spend() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");

        assert_eq!(
            service
                .require_spend_allowed_with_default_ceo_slot("member-uid", "default", false)
                .expect_err("non slot player cannot spend"),
            OrganizationError::NotCeo
        );
    }

    #[test]
    fn player_org_ceo_can_perform_admin_actions() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_player_org("org-1", "Player Org", "ceo-uid")
            .expect("player org should be created");

        assert!(
            service
                .require_action_allowed("ceo-uid", "org-1", OrganizationAction::Rename)
                .is_ok()
        );
    }

    #[test]
    fn member_can_access_but_cannot_spend() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        let mut organization = Organization::player_org("org-1", "Player Org", "ceo-uid");
        organization.members.push(OrganizationMember::new(
            "member-uid",
            OrganizationRole::Member,
        ));
        service
            .repository
            .save(organization)
            .expect("org should be saved");

        assert!(service.require_member("member-uid", "org-1").is_ok());
        assert_eq!(
            service
                .require_spend_allowed("member-uid", "org-1")
                .expect_err("member cannot spend"),
            OrganizationError::NotCeo
        );
    }

    #[test]
    fn player_org_ceo_cannot_leave_without_disbanding() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_player_org("org-1", "Player Org", "ceo-uid")
            .expect("player org should be created");

        assert_eq!(
            service
                .leave_member("org-1", "ceo-uid")
                .expect_err("ceo should disband instead of leave"),
            OrganizationError::LastCeoCannotLeave
        );
    }

    #[test]
    fn player_org_member_leave_moves_member_to_default_org() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        let mut organization = Organization::player_org("org-1", "Player Org", "ceo-uid");
        organization.members.push(OrganizationMember::new(
            "member-uid",
            OrganizationRole::Member,
        ));
        service.repository.save(organization).unwrap();

        let transfer = service
            .leave_member("org-1", "member-uid")
            .expect("member should leave");

        assert_eq!(transfer.organization.id, "org-1");
        assert_eq!(transfer.default_organization.id, "default");
        assert_eq!(transfer.uid, "member-uid");
        assert!(
            !service
                .get("org-1")
                .unwrap()
                .unwrap()
                .has_member("member-uid")
        );
        assert!(
            service
                .get("default")
                .unwrap()
                .unwrap()
                .has_member("member-uid")
        );
    }

    #[test]
    fn default_org_member_cannot_leave_or_be_kicked() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        service.add_member("default", "member-uid").unwrap();

        assert_eq!(
            service
                .leave_member("default", "member-uid")
                .expect_err("default member cannot leave"),
            OrganizationError::CannotLeaveDefaultOrg
        );
        assert_eq!(
            service
                .kick_member("default", "ceo-uid", "member-uid")
                .expect_err("default member cannot be kicked"),
            OrganizationError::RestrictedDefaultOrgAction
        );
    }

    #[test]
    fn kick_member_moves_member_to_default_org() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        let mut organization = Organization::player_org("org-1", "Player Org", "ceo-uid");
        organization.members.push(OrganizationMember::new(
            "member-uid",
            OrganizationRole::Member,
        ));
        service.repository.save(organization).unwrap();

        let transfer = service
            .kick_member("org-1", "ceo-uid", "member-uid")
            .expect("member should be kicked");

        assert_eq!(transfer.uid, "member-uid");
        assert!(
            !service
                .get("org-1")
                .unwrap()
                .unwrap()
                .has_member("member-uid")
        );
        assert!(
            service
                .get("default")
                .unwrap()
                .unwrap()
                .has_member("member-uid")
        );
    }

    #[test]
    fn create_player_org_moves_ceo_from_default_org() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        service.add_member("default", "ceo-uid").unwrap();

        let organization = service
            .create_player_org("org-1", "Player Org", "ceo-uid")
            .expect("player org should be created");

        assert!(organization.is_ceo("ceo-uid"));
        assert!(
            !service
                .get("default")
                .unwrap()
                .unwrap()
                .has_member("ceo-uid")
        );
    }

    #[test]
    fn accept_invite_moves_member_from_previous_org() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        service.add_member("default", "member-uid").unwrap();
        service
            .create_player_org("org-1", "Player Org", "ceo-uid")
            .expect("player org should be created");
        let invite = service
            .create_invite("ceo-uid", "org-1", "member-uid")
            .expect("invite should be created");

        let (organization, _) = service
            .accept_invite("member-uid", &invite.id.to_string())
            .expect("invite should be accepted");

        assert!(organization.has_member("member-uid"));
        assert!(
            !service
                .get("default")
                .unwrap()
                .unwrap()
                .has_member("member-uid")
        );
    }

    #[test]
    fn disband_player_org_reassigns_members_to_default_org() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org()
            .expect("default org should be created");
        let mut organization = Organization::player_org("org-1", "Player Org", "ceo-uid");
        organization.members.push(OrganizationMember::new(
            "member-uid",
            OrganizationRole::Member,
        ));
        service.repository.save(organization).unwrap();

        let disband = service
            .disband_player_org("org-1", "ceo-uid")
            .expect("player org should disband");

        assert_eq!(disband.disbanded.id, "org-1");
        assert_eq!(disband.reassigned_uids, ["ceo-uid", "member-uid"]);
        assert!(service.get("org-1").expect("lookup should work").is_none());
        let default_org = service
            .get("default")
            .expect("lookup should work")
            .expect("default org should exist");
        assert!(default_org.has_member("ceo-uid"));
        assert!(default_org.has_member("member-uid"));
        assert!(!default_org.is_ceo("ceo-uid"));
    }

    #[test]
    fn default_org_ceo_slot_can_issue_payday_to_members() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org_with_starting("100.00", &VGarage::default(), &VLocker::default())
            .expect("default org should be created");
        service
            .add_member("default", "ceo-uid")
            .expect("issuer should be added");
        service
            .add_member("default", "member-1")
            .expect("member should be added");
        service
            .add_member("default", "member-2")
            .expect("member should be added");

        let payday = service
            .issue_payday("ceo-uid", "default", "25.00", true)
            .expect("payday should be issued");

        assert_eq!(payday.amount.as_str(), "25.00");
        assert_eq!(payday.recipients, ["member-1", "member-2"]);
        assert_eq!(payday.organization.bank.as_str(), "50.00");
    }

    #[test]
    fn payday_uses_org_members() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org_with_starting("100.00", &VGarage::default(), &VLocker::default())
            .expect("default org should be created");
        service
            .add_member("default", "ceo-uid")
            .expect("issuer should be added");
        service
            .add_member("default", "member-1")
            .expect("member should be added");
        service
            .add_member("default", "member-2")
            .expect("member should be added");

        let payday = service
            .issue_payday("ceo-uid", "default", "25.00", true)
            .expect("payday should be issued");

        assert_eq!(payday.recipients, ["member-1", "member-2"]);
        assert_eq!(payday.organization.bank.as_str(), "50.00");
    }

    #[test]
    fn payday_rejects_insufficient_org_funds() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_default_org_with_starting("10.00", &VGarage::default(), &VLocker::default())
            .expect("default org should be created");
        service
            .add_member("default", "member-1")
            .expect("member should be added");

        assert_eq!(
            service
                .issue_payday("ceo-uid", "default", "25.00", true)
                .expect_err("payday should fail"),
            OrganizationError::InsufficientFunds
        );
    }

    #[test]
    fn player_org_payday_uses_member_recipients() {
        let service = OrganizationService::new(InMemoryOrganizationRepository::new());
        service
            .create_player_org("org-1", "Player Org", "ceo-uid")
            .expect("player org should be created");
        let mut organization = service.get("org-1").unwrap().unwrap();
        organization.bank = Money::from_cents(10_000);
        organization.members.push(OrganizationMember::new(
            "member-uid",
            OrganizationRole::Member,
        ));
        service.repository.save(organization).unwrap();

        let payday = service
            .issue_payday("ceo-uid", "org-1", "25.00", false)
            .expect("payday should be issued");

        assert_eq!(payday.recipients, ["member-uid"]);
        assert_eq!(payday.organization.bank.as_str(), "75.00");
    }
}
