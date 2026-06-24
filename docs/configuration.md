# Configuration

Forge configuration is split across extension TOML, mission config, CBA settings, and Eden object naming.

## Extension TOML

Source example:

```text
arma/crate/config.example.toml
```

Current database fields:

| Field | Purpose |
| --- | --- |
| `enabled` | Enables SurrealDB and the background persistence worker. |
| `endpoint` | SurrealDB WebSocket endpoint, without an implicit protocol rewrite. |
| `namespace` | SurrealDB namespace. |
| `database` | SurrealDB database. |
| `username` | Root authentication username. |
| `password` | Root authentication password. |
| `channel_capacity` | Bounded asynchronous write queue capacity. |
| `reconnect_initial_ms` | Initial reconnect delay. |
| `reconnect_max_ms` | Maximum exponential-backoff delay. |

Environment overrides:

| Variable | Purpose |
| --- | --- |
| `FORGE_SERVER_CONFIG` | Exact TOML config path. |
| `FORGE_SERVER_LOG_DIR` | Directory for aggregate and domain logs. |
| `FORGE_SERVER_LOG` | Exact aggregate log path; its parent becomes the domain-log directory. |

## Mission Configuration

Actor initialization reads:

```text
CfgMission >> Actor
```

The SQF actor snapshot supplies the configured default loadout and starting values to Rust when creating a new actor. Existing actors remain authoritative after cache hydration.

Paid gameplay services read:

```text
CfgMission >> Services >> Refuel
CfgMission >> Services >> Repair
CfgMission >> Services >> Rearm
CfgMission >> Services >> Medical
```

Supported values include:

- `regularPricePerLiter`
- `jeta1PricePerLiter`
- `fullRepairPrice`
- `unitPrice`
- `respawnPrice`
- `fullHealPrice`

A zero fee is valid and completes the service without a bank withdrawal.

## CBA Settings

Actor:

| Setting | Default | Effect |
| --- | --- | --- |
| `forge_crate_actor_persistPosition` | Enabled | Restores and updates persisted actor position. |
| `forge_crate_actor_persistLoadout` | Enabled | Restores and updates persisted actor loadout. |

Virtual Garage:

| Setting | Default | Effect |
| --- | --- | --- |
| `forge_crate_v_garage_enabled` | Enabled | Enables the virtual garage feature. |
| `forge_crate_v_garage_persistenceEnabled` | Enabled | Enables persistent unlock storage. |

Virtual Locker:

| Setting | Default | Effect |
| --- | --- | --- |
| `forge_crate_v_locker_enabled` | Enabled | Enables the ACE Arsenal integration. |
| `forge_crate_v_locker_persistenceEnabled` | Enabled | Enables persistent unlock storage. |

These are globally synchronized settings.

## Eden Object Naming

Bank terminals:

```text
bank
bank_1
...
bank_999
```

Locker and Virtual Arsenal terminals:

```text
locker
locker_1
...
locker_999
```

Registration is idempotent. Each valid client-side object receives the relevant `addAction` once.
