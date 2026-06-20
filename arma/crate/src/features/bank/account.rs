use super::BankFeature;
use forge_lib::{
    models::{Money, PlayerBankProfileView},
    repositories::BankRepository,
    shared::BankError,
};

impl<R> BankFeature<R>
where
    R: BankRepository,
{
    pub(crate) fn init_player_account(
        &self,
        uid: &str,
        starting_cash: &str,
        starting_bank: &str,
    ) -> Result<PlayerBankProfileView, BankError> {
        self.service
            .init_player_account(uid, starting_cash, starting_bank)
    }

    pub(crate) fn get_account(
        &self,
        uid: &str,
    ) -> Result<Option<PlayerBankProfileView>, BankError> {
        self.service.get_account(uid)
    }

    pub(crate) fn deposit_to_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        self.service.deposit_to_account(uid, amount)
    }

    pub(crate) fn withdraw_from_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        self.service.withdraw_from_account(uid, amount)
    }

    pub(crate) fn transfer_between_accounts(
        &self,
        from_uid: &str,
        to_uid: &str,
        amount: Money,
    ) -> Result<(PlayerBankProfileView, PlayerBankProfileView), BankError> {
        self.service
            .transfer_between_accounts(from_uid, to_uid, amount)
    }
}
