use crate::{features::medical::MedicalFeature, log};
use arma_rs::Group;
use forge_lib::services::{BankService, MedicalService};
use std::sync::LazyLock;

static MEDICAL_FEATURE: LazyLock<MedicalFeature<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| {
        MedicalFeature::new(MedicalService::new(BankService::new(
            crate::persistence::bank_repository(),
        )))
    });

pub fn group() -> Group {
    Group::new()
        .command("respawn", medical_respawn)
        .command("heal", medical_heal)
}

pub(crate) fn medical_respawn(uid: String) -> String {
    match MEDICAL_FEATURE.record_respawn(&uid) {
        Ok(receipt) => serialize_medical(&receipt, "medical respawn receipt"),
        Err(error) => {
            log::error(format_args!("failed to record respawn for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn medical_heal(uid: String) -> String {
    match MEDICAL_FEATURE.full_heal(&uid) {
        Ok(receipt) => serialize_medical(&receipt, "medical heal receipt"),
        Err(error) => {
            log::error(format_args!("failed to heal {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn serialize_medical<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
