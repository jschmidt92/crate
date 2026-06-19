# Feature Guide

This page summarizes the current Rust server features and the main files involved.

## Actor

Main files:

- `lib/src/models/actor.rs`
- `lib/src/services/actor.rs`
- `arma/server/src/actor.rs`
- `arma/server/src/features/actor/*`

Actor init accepts an `ActorSnapshot`. If the actor is new, `ActorService::init_or_create` creates it and returns an `ActorCreated` domain event. The actor feature publishes that event through the central event bus.

Actor server workflows are organized as vertical slices:

- `init.rs`: init or create actor.
- `lifecycle.rs`: disconnect and delete actor.
- `query.rs`: get actor by uid.

Current commands:

- `actor:init`
- `actor:disconnect`
- `actor:disconnect_uid`
- `actor:get`
- `actor:delete`

## Bank

Main files:

- `lib/src/models/bank.rs`
- `lib/src/services/bank.rs`
- `lib/src/repositories/bank.rs`
- `arma/server/src/bank.rs`
- `arma/server/src/features/bank/*`
- `arma/server/src/persistence/payday.rs`

Bank profiles hold player cash and account balances. Player bank-account reads and money movement go through `BankService`. Organization payday still applies the organization debit and recipient bank credits as one queued SurrealDB transaction batch, but recipient bank credits are prepared through `BankService` before persistence batches the writes.

Bank server workflows are organized as:

- `account.rs`: initialize, read, deposit, withdraw, and transfer player bank funds.
- `lifecycle.rs`: disconnect player bank profile.

Current commands:

- `bank:init`
- `bank:get`
- `bank:deposit`
- `bank:withdraw`
- `bank:transfer`
- `bank:disconnect`

## Garage and Locker

Main files:

- `lib/src/models/garage.rs`
- `lib/src/models/locker.rs`
- `lib/src/models/v_garage.rs`
- `lib/src/models/v_locker.rs`
- `lib/src/services/garage.rs`
- `lib/src/services/locker.rs`
- `lib/src/services/v_garage.rs`
- `lib/src/services/v_locker.rs`
- `arma/server/src/garage.rs`
- `arma/server/src/locker.rs`
- `arma/server/src/v_garage.rs`
- `arma/server/src/v_locker.rs`
- `arma/server/src/features/garage/*`
- `arma/server/src/features/locker/*`
- `arma/server/src/features/v_garage/*`
- `arma/server/src/features/v_locker/*`

Physical garage/locker data and virtual unlock collections are separate models and repositories.

These server workflows use the same slice names:

- `lifecycle.rs`: initialize, disconnect, and delete records.
- `query.rs`: get records by player uid.
- `storage.rs`: save full records.

Current command groups:

- `garage:*`
- `locker:*`
- `v_garage:*`
- `v_locker:*`

## Refuel

Main files:

- `lib/src/models/fuel.rs`
- `lib/src/models/service.rs`
- `lib/src/models/transaction.rs`
- `lib/src/services/refuel.rs`
- `arma/server/src/refuel.rs`
- `arma/server/src/features/refuel/*`

Refuel supports session-based refueling from Arma events and direct refuel completion commands. Completed refuels charge the player bank account through `BankService` and return a `ServiceReceipt`. Refuel prices are read from `CfgMission >> Services >> Refuel`, with Rust defaults used by the domain service if a caller does not provide custom pricing.

```mermaid
flowchart TD
    Started[refuel:started] --> Session[store fueling session]
    Tick[refuel:tick] --> Session
    Stopped[refuel:stopped] --> Complete[FuelService::complete_with_price]
    Complete --> Bank[BankService::withdraw_from_account]
    Bank --> Receipt[ServiceReceipt]
```

Current commands:

- `refuel:started`
- `refuel:tick`
- `refuel:stopped`
- `refuel:price`
- `refuel:quote`
- `refuel:complete`

## Gameplay Services

Main files:

- `lib/src/models/service.rs`
- `lib/src/services/repair.rs`
- `lib/src/services/rearm.rs`
- `lib/src/services/medical.rs`
- `arma/server/src/repair.rs`
- `arma/server/src/rearm.rs`
- `arma/server/src/medical.rs`
- `arma/server/src/features/repair/*`
- `arma/server/src/features/rearm/*`
- `arma/server/src/features/medical/*`

Repair, rearm, refuel, and medical services are service-style workflows. They validate the requested work, calculate a quote, charge the player bank account through `BankService` when the configured fee is greater than zero, and return a consistent `ServiceReceipt`.

Mission-config pricing:

- `CfgMission >> Services >> Refuel >> regularPricePerLiter`
- `CfgMission >> Services >> Refuel >> jeta1PricePerLiter`
- `CfgMission >> Services >> Repair >> fullRepairPrice`
- `CfgMission >> Services >> Rearm >> unitPrice`
- `CfgMission >> Services >> Medical >> respawnPrice`
- `CfgMission >> Services >> Medical >> fullHealPrice`

If a configured value is `0.00`, the service completes without a bank withdrawal. Rust services keep internal defaults so direct callers still have a deterministic fallback.

Current commands:

- `repair:quote`
- `repair:complete`
- `rearm:quote`
- `rearm:complete`
- `medical:respawn`
- `medical:heal`

## Organization

Main files:

- `lib/src/models/organization.rs`
- `lib/src/models/organization_event.rs`
- `lib/src/services/organization.rs`
- `lib/src/repositories/organization.rs`
- `arma/server/src/organization.rs`
- `arma/server/src/features/organization/*`

Organization server workflows are organized as vertical slices:

- `create.rs`: create default org, create player org, disband player org.
- `invite.rs`: create, accept, and decline invites.
- `membership.rs`: leave org, kick member, add member.
- `payday.rs`: issue payday.
- `query.rs`: get organization by id or member uid.

### Organization Rules

- The default organization is the fallback organization.
- Players cannot directly leave the default organization.
- Players cannot be kicked from the default organization.
- A player leaves the default organization only by accepting an invite or creating their own player organization.
- A player can belong to one organization at a time.
- Creating a player organization moves the new CEO out of their previous organization.
- Accepting an invite moves the player out of their previous organization before adding them to the invited organization.
- A player organization has one CEO.
- The CEO cannot leave a player organization. The CEO must disband it.
- Disbanding a player organization moves all former members, including the CEO, into the default organization as regular members.
- Members can leave a player organization and are moved to the default organization.
- CEOs can kick non-CEO members from a player organization, and kicked members are moved to the default organization.

### Organization Payday

Payday is split into two phases:

1. `OrganizationService::prepare_payday` validates permissions, recipients, amount, and organization balance.
2. `BankService::prepare_deposits` prepares recipient bank credits.
3. `persistence::apply_payday_plan` applies the organization debit and all recipient bank credits as a queued transaction batch.

After the money movement is applied in memory and queued for persistence, the server publishes `OrganizationPaydayIssued`. The durable event backend records the event, an audit row, and recipient notifications.

Current commands:

- `organization:create_default`
- `organization:create_player`
- `organization:disband`
- `organization:create_invite`
- `organization:accept_invite`
- `organization:decline_invite`
- `organization:leave_member`
- `organization:kick_member`
- `organization:add_member`
- `organization:get`
- `organization:get_by_member`
- `organization:issue_payday`

## Persistence Status and Transport

Main files:

- `arma/server/src/config.rs`
- `arma/server/src/transport.rs`
- `arma/server/src/persistence/*`

Useful commands:

- `database_status`
- `config_path`
- `log_path`
- `transport:*`
