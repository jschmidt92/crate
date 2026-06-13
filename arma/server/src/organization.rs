use crate::{events, log};
use arma_rs::Group;
use forge_lib::{
    models::{
        DomainEvent, OrganizationCreated, OrganizationDisbanded, OrganizationInviteAccepted,
        OrganizationInviteCreated, OrganizationInviteDeclined, OrganizationMemberKicked,
        OrganizationMemberLeft, OrganizationPaydayIssued, OrganizationView, VGarage, VLocker,
    },
    services::OrganizationService,
};
use std::sync::LazyLock;

static ORGANIZATION_SERVICE: LazyLock<
    OrganizationService<crate::persistence::CachedOrganizationRepository>,
> = LazyLock::new(|| OrganizationService::new(crate::persistence::organization_repository()));

pub fn group() -> Group {
    Group::new()
        .command("create_default", create_default)
        .command("create_player", create_player)
        .command("disband", disband)
        .command("create_invite", create_invite)
        .command("accept_invite", accept_invite)
        .command("decline_invite", decline_invite)
        .command("leave_member", leave_member)
        .command("kick_member", kick_member)
        .command("add_member", add_member)
        .command("get", get_organization)
        .command("get_by_member", get_by_member)
        .command("issue_payday", issue_payday)
}

pub(crate) fn create_default(
    starting_bank: String,
    virtual_garage_json: String,
    virtual_locker_json: String,
) -> String {
    let virtual_garage = match serde_json::from_str::<VGarage>(&virtual_garage_json) {
        Ok(virtual_garage) => virtual_garage,
        Err(error) => {
            log::error(format_args!(
                "invalid default organization virtual garage payload: {error}"
            ));
            return format!("Error: invalid default organization virtual garage payload: {error}");
        }
    };

    let virtual_locker = match serde_json::from_str::<VLocker>(&virtual_locker_json) {
        Ok(virtual_locker) => virtual_locker,
        Err(error) => {
            log::error(format_args!(
                "invalid default organization virtual locker payload: {error}"
            ));
            return format!("Error: invalid default organization virtual locker payload: {error}");
        }
    };

    match ORGANIZATION_SERVICE.create_default_org_with_starting(
        &starting_bank,
        &virtual_garage,
        &virtual_locker,
    ) {
        Ok(organization) => serialize_organization(&organization),
        Err(error) => {
            log::error(format_args!("failed to create default org: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn create_player(id: String, name: String, ceo_uid: String) -> String {
    match ORGANIZATION_SERVICE.create_player_org(&id, &name, &ceo_uid) {
        Ok(organization) => {
            events::publish(DomainEvent::OrganizationCreated(OrganizationCreated::new(
                OrganizationView::from(&organization),
                &ceo_uid,
            )));
            serialize_organization(&organization)
        }
        Err(error) => {
            log::error(format_args!("failed to create player org {id}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disband(organization_id: String, ceo_uid: String) -> String {
    match ORGANIZATION_SERVICE.disband_player_org(&organization_id, &ceo_uid) {
        Ok(disband) => {
            events::publish(DomainEvent::OrganizationDisbanded(
                OrganizationDisbanded::new(
                    disband.disbanded.clone(),
                    &ceo_uid,
                    disband.reassigned_uids.clone(),
                ),
            ));
            serialize(&disband, "organization disband")
        }
        Err(error) => {
            log::error(format_args!(
                "failed to disband org {organization_id} by {ceo_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn create_invite(
    inviter_uid: String,
    organization_id: String,
    invitee_uid: String,
) -> String {
    match ORGANIZATION_SERVICE.create_invite(&inviter_uid, &organization_id, &invitee_uid) {
        Ok(invite) => {
            events::publish(DomainEvent::OrganizationInviteCreated(
                OrganizationInviteCreated::new(invite.clone()),
            ));
            serialize(&invite, "organization invite")
        }
        Err(error) => {
            log::error(format_args!(
                "failed to create invite for {invitee_uid} to org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn accept_invite(invitee_uid: String, invite_id: String) -> String {
    match ORGANIZATION_SERVICE.accept_invite(&invitee_uid, &invite_id) {
        Ok((organization, invite)) => {
            events::publish(DomainEvent::OrganizationInviteAccepted(
                OrganizationInviteAccepted::new(invite),
            ));
            serialize_organization(&organization)
        }
        Err(error) => {
            log::error(format_args!(
                "failed to accept invite {invite_id} for {invitee_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn decline_invite(invitee_uid: String, invite_id: String) -> String {
    match ORGANIZATION_SERVICE.decline_invite(&invitee_uid, &invite_id) {
        Ok(invite) => {
            events::publish(DomainEvent::OrganizationInviteDeclined(
                OrganizationInviteDeclined::new(invite.clone()),
            ));
            serialize(&invite, "organization invite")
        }
        Err(error) => {
            log::error(format_args!(
                "failed to decline invite {invite_id} for {invitee_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn leave_member(organization_id: String, uid: String) -> String {
    match ORGANIZATION_SERVICE.leave_member(&organization_id, &uid) {
        Ok(transfer) => {
            events::publish(DomainEvent::OrganizationMemberLeft(
                OrganizationMemberLeft::new(
                    transfer.organization.clone(),
                    transfer.default_organization.clone(),
                    &uid,
                ),
            ));
            serialize(&transfer, "organization member transfer")
        }
        Err(error) => {
            log::error(format_args!(
                "failed to remove member {uid} from org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn kick_member(
    organization_id: String,
    actor_uid: String,
    kicked_uid: String,
) -> String {
    match ORGANIZATION_SERVICE.kick_member(&organization_id, &actor_uid, &kicked_uid) {
        Ok(transfer) => {
            events::publish(DomainEvent::OrganizationMemberKicked(
                OrganizationMemberKicked::new(
                    transfer.organization.clone(),
                    transfer.default_organization.clone(),
                    &actor_uid,
                    &kicked_uid,
                ),
            ));
            serialize(&transfer, "organization member transfer")
        }
        Err(error) => {
            log::error(format_args!(
                "failed to kick member {kicked_uid} from org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn add_member(organization_id: String, uid: String) -> String {
    match ORGANIZATION_SERVICE.add_member(&organization_id, &uid) {
        Ok(organization) => serialize_organization(&organization),
        Err(error) => {
            log::error(format_args!(
                "failed to add member {uid} to org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_organization(id: String) -> String {
    match ORGANIZATION_SERVICE.get(&id) {
        Ok(Some(organization)) => serialize_organization(&organization),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get org {id}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_by_member(uid: String) -> String {
    match ORGANIZATION_SERVICE.get_by_member_uid(&uid) {
        Ok(Some(organization)) => serialize_organization(&organization),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get org for member {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn issue_payday(
    uid: String,
    organization_id: String,
    amount: String,
    in_default_ceo_slot: String,
) -> String {
    let plan = match ORGANIZATION_SERVICE.prepare_payday(
        &uid,
        &organization_id,
        &amount,
        parse_bool(&in_default_ceo_slot),
    ) {
        Ok(payday) => payday,
        Err(error) => {
            log::error(format_args!(
                "failed to issue payday for org {organization_id}: {error}"
            ));
            return format!("Error: {error}");
        }
    };

    let payday = match crate::persistence::apply_payday_plan(plan) {
        Ok(payday) => payday,
        Err(error) => {
            log::error(format_args!(
                "failed to apply payday for org {organization_id}: {error}"
            ));
            return format!("Error: {error}");
        }
    };

    events::publish(DomainEvent::OrganizationPaydayIssued(
        OrganizationPaydayIssued::new(
            payday.organization.clone(),
            &uid,
            payday.amount.clone(),
            payday.recipients.clone(),
        ),
    ));

    match serde_json::to_string(&payday) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize payday result: {error}"));
            format!("Error: failed to serialize payday result: {error}")
        }
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value, "true" | "1" | "yes")
}

fn serialize_organization(organization: &forge_lib::models::Organization) -> String {
    serialize(&OrganizationView::from(organization), "organization")
}

fn serialize<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    match serde_json::to_string(value) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize {label}: {error}"));
            format!("Error: failed to serialize {label}: {error}")
        }
    }
}
