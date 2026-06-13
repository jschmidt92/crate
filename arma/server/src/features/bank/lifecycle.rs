use super::BankFeature;
use forge_lib::{repositories::BankRepository, shared::BankError};

impl<R> BankFeature<R>
where
    R: BankRepository,
{
    pub(crate) fn disconnect_player_account(&self, uid: &str) -> Result<(), BankError> {
        self.service.disconnect_player_account(uid)
    }
}
