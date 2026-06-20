use super::{BANK_REPOSITORY, ORGANIZATION_REPOSITORY, enqueue, model::WriteOp, upsert_op};
use forge_lib::{
    models::{OrganizationPayday, OrganizationPaydayPlan, OrganizationView},
    services::BankService,
    shared::BankError,
};

pub fn apply_payday_plan(plan: OrganizationPaydayPlan) -> Result<OrganizationPayday, BankError> {
    let mut ops = Vec::new();
    let organization = ORGANIZATION_REPOSITORY
        .save_without_enqueue(plan.organization)
        .map_err(|error| BankError::Repository(error.to_string()))?;

    ops.push(upsert_op("organization", &organization.id, &organization)?);

    let bank_service = BankService::new(BANK_REPOSITORY.clone());
    let deposits = plan
        .recipients
        .iter()
        .map(|uid| (uid.clone(), plan.amount))
        .collect::<Vec<_>>();

    for profile in bank_service.prepare_deposits(&deposits)? {
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
