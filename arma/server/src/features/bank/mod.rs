mod account;
mod lifecycle;

use forge_lib::{repositories::BankRepository, services::BankService};

#[derive(Clone)]
pub(crate) struct BankFeature<R> {
    service: BankService<R>,
}

impl<R> BankFeature<R>
where
    R: BankRepository,
{
    pub(crate) const fn new(service: BankService<R>) -> Self {
        Self { service }
    }
}
