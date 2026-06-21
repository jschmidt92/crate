# Persistence Status and Transport Feature

Main files:
- `arma/crate/src/config.rs`
- `arma/crate/src/transport.rs`
- `arma/crate/src/persistence/*`

## Mechanics
This feature manages database connection statuses, system configuration paths, logging paths, and raw data transmission/transport formats between the game client, server extension, and the backend persistence database.

## Current Commands
- `database_status`
- `config_path`
- `log_path`
- `transport:*`
