# Bank Feature

Main files:
- `lib/src/models/bank.rs`
- `lib/src/services/bank.rs`
- `lib/src/repositories/bank.rs`
- `arma/crate/src/bank.rs`
- `arma/crate/src/features/bank/*`
- `arma/crate/src/persistence/payday.rs`

## Mechanics

### Bank Profiles
Bank profiles hold player cash, account balances, pending earnings, a salted ATM PIN hash, and up to ten recent ledger entries. Player bank-account reads and money movement go through `BankService`. 

### Money Transfers
Player transfers persist the sender debit and recipient credit in one queued transaction batch. Organization payday applies the organization debit and recipient bank credits the same way, with recipient credits prepared through `BankService` before persistence batches the writes.

### WebUI Requests
The bank WebUI uses a server-authoritative request/response bridge. Browser requests travel through `JSDialog` to SQF, are forwarded to the server, and call the Rust extension using the requesting player's UID. The resulting bank snapshot is returned to that player's browser with `ctrlWebBrowserAction ["ExecJS", ...]`. The UI does not update balances optimistically.

## Code Organization
Bank server workflows are organized as:
- `account.rs`: Initialize, read, deposit, withdraw, and transfer player bank funds.

Bank disconnect cleanup is an internal `ActorDisconnected` event handler rather than a public extension command.

## Current Commands
- `bank:init`
- `bank:get`
- `bank:deposit`
- `bank:withdraw`
- `bank:transfer`
- `bank:add_earnings`
- `bank:submit_earnings`
- `bank:change_pin`
