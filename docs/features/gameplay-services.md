# Gameplay Services

Main files:
- `lib/src/models/service.rs`
- `lib/src/services/repair.rs`
- `lib/src/services/rearm.rs`
- `lib/src/services/medical.rs`
- `arma/crate/src/repair.rs`
- `arma/crate/src/rearm.rs`
- `arma/crate/src/medical.rs`
- `arma/crate/src/features/repair/*`
- `arma/crate/src/features/rearm/*`
- `arma/crate/src/features/medical/*`

## Mechanics

### Service Workflows
Repair, rearm, refuel, and medical services are service-style workflows. They validate the requested work, calculate a quote, charge the player bank account through `BankService` when the configured fee is greater than zero, and return a consistent `ServiceReceipt`.

### Mission Configuration Pricing
Pricing configuration is defined in the mission configs:
- `CfgMission >> Services >> Refuel >> regularPricePerLiter`
- `CfgMission >> Services >> Refuel >> jeta1PricePerLiter`
- `CfgMission >> Services >> Repair >> fullRepairPrice`
- `CfgMission >> Services >> Rearm >> unitPrice`
- `CfgMission >> Services >> Medical >> respawnPrice`
- `CfgMission >> Services >> Medical >> fullHealPrice`

If a configured value is `0.00`, the service completes without a bank withdrawal. Rust services keep internal defaults so direct callers still have a deterministic fallback.

## Current Commands
- `repair:quote`
- `repair:complete`
- `rearm:quote`
- `rearm:complete`
- `medical:respawn`
- `medical:heal`
