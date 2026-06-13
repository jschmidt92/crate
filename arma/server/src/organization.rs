use crate::{bank, log};
use arma_rs::Group;
use forge_lib::{
    models::{OrganizationView, VGarage, VLocker},
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
        Ok(organization) => serialize_organization(&organization),
        Err(error) => {
            log::error(format_args!("failed to create player org {id}: {error}"));
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
    let payday = match ORGANIZATION_SERVICE.issue_payday(
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

    let amount = match payday.amount.parse() {
        Some(amount) => amount,
        None => return "Error: invalid payday amount".to_string(),
    };

    for recipient_uid in &payday.recipients {
        if let Err(error) = bank::deposit_payday(recipient_uid, amount) {
            log::error(format_args!(
                "failed to deposit payday for recipient {recipient_uid}: {error}"
            ));
            return format!(
                "Error: failed to deposit payday for recipient {recipient_uid}: {error}"
            );
        }
    }

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
    match serde_json::to_string(&OrganizationView::from(organization)) {
        Ok(json) => json,
        Err(error) => {
            log::error(format_args!("failed to serialize organization: {error}"));
            format!("Error: failed to serialize organization: {error}")
        }
    }
}
