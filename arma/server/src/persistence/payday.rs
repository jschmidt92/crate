use super::{BANK_REPOSITORY, ORGANIZATION_REPOSITORY, enqueue, model::WriteOp, upsert_op};
use forge_lib::{
    models::{OrganizationPayday, OrganizationPaydayPlan, OrganizationView, PlayerBankProfile},
    repositories::BankRepository,
    shared::BankError,
};

pub fn apply_payday_plan(plan: OrganizationPaydayPlan) -> Result<OrganizationPayday, BankError> {
    let mut ops = Vec::new();
    let organization = ORGANIZATION_REPOSITORY
        .save_without_enqueue(plan.organization)
        .map_err(|error| BankError::Repository(error.to_string()))?;

    ops.push(upsert_op("organization", &organization.id, &organization)?);

    for recipient_uid in &plan.recipients {
        let mut profile = BANK_REPOSITORY
            .find_by_uid(recipient_uid)
            .map_err(|error| BankError::Repository(error.to_string()))?
            .unwrap_or_else(|| PlayerBankProfile::new(recipient_uid));
        profile.account.deposit(plan.amount);
        let profile = BANK_REPOSITORY.save_without_enqueue(profile)?;
        ops.push(upsert_op("bank", &profile.uid, &profile)?);
    }

    enqueue(WriteOp::Batch { ops });

    Ok(OrganizationPayday {
        organization: OrganizationView::from(&organization),
        amount: plan.amount.to_amount(),
        recipients: plan.recipients,
    })
}
