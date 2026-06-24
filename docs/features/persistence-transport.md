# Persistence Status and Transport Feature

Main files:
- `arma/crate/src/config.rs`
- `arma/crate/src/transport.rs`
- `arma/crate/src/persistence/*`

## Mechanics
This feature manages database readiness/status, system configuration paths, logging paths, and chunked request/response transport.

Actor initialization uses `database_ready`, not merely `database_status`, to avoid querying empty caches during cold-start hydration.

The SQF extension wrapper sends small requests directly through `transport:invoke`. Large JSON arguments are staged in chunks, invoked from stored request data, and large responses are fetched and reassembled in chunks.

## Current Commands
- `database_status`
- `database_ready`
- `config_path`
- `log_path`
- `transport:*`
