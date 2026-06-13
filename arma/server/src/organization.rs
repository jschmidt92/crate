use crate::{
    events::ServerEventPublisher,
    features::organization::{OrganizationFeature, PersistencePaydayApplier},
    log,
};
use arma_rs::Group;
use forge_lib::{
    models::{OrganizationView, VGarage, VLocker},
    services::OrganizationService,
};
use std::sync::LazyLock;

static ORGANIZATION_APP: LazyLock<
    OrganizationFeature<
        crate::persistence::CachedOrganizationRepository,
        ServerEventPublisher,
        PersistencePaydayApplier,
    >,
> = LazyLock::new(|| {
    OrganizationFeature::new(
        OrganizationService::new(crate::persistence::organization_repository()),
        ServerEventPublisher,
        PersistencePaydayApplier,
    )
});

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

    match ORGANIZATION_APP.create_default_org_with_starting(
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
    match ORGANIZATION_APP.create_player_org(&id, &name, &ceo_uid) {
        Ok(organization) => serialize_organization(&organization),
        Err(error) => {
            log::error(format_args!("failed to create player org {id}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disband(organization_id: String, ceo_uid: String) -> String {
    match ORGANIZATION_APP.disband_player_org(&organization_id, &ceo_uid) {
        Ok(disband) => serialize(&disband, "organization disband"),
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
    match ORGANIZATION_APP.create_invite(&inviter_uid, &organization_id, &invitee_uid) {
        Ok(invite) => serialize(&invite, "organization invite"),
        Err(error) => {
            log::error(format_args!(
                "failed to create invite for {invitee_uid} to org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn accept_invite(invitee_uid: String, invite_id: String) -> String {
    match ORGANIZATION_APP.accept_invite(&invitee_uid, &invite_id) {
        Ok(organization) => serialize_organization(&organization),
        Err(error) => {
            log::error(format_args!(
                "failed to accept invite {invite_id} for {invitee_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn decline_invite(invitee_uid: String, invite_id: String) -> String {
    match ORGANIZATION_APP.decline_invite(&invitee_uid, &invite_id) {
        Ok(invite) => serialize(&invite, "organization invite"),
        Err(error) => {
            log::error(format_args!(
                "failed to decline invite {invite_id} for {invitee_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn leave_member(organization_id: String, uid: String) -> String {
    match ORGANIZATION_APP.leave_member(&organization_id, &uid) {
        Ok(transfer) => serialize(&transfer, "organization member transfer"),
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
    match ORGANIZATION_APP.kick_member(&organization_id, &actor_uid, &kicked_uid) {
        Ok(transfer) => serialize(&transfer, "organization member transfer"),
        Err(error) => {
            log::error(format_args!(
                "failed to kick member {kicked_uid} from org {organization_id}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn add_member(organization_id: String, uid: String) -> String {
    match ORGANIZATION_APP.add_member(&organization_id, &uid) {
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
    match ORGANIZATION_APP.get(&id) {
        Ok(Some(organization)) => serialize_organization(&organization),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get org {id}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_by_member(uid: String) -> String {
    match ORGANIZATION_APP.get_by_member_uid(&uid) {
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
    let payday = match ORGANIZATION_APP.issue_payday(
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

    serde_json::to_string(&payday).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize payday result: {error}"));
        format!("Error: failed to serialize payday result: {error}")
    })
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
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
