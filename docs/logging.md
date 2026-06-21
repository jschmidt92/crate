# Extension Logging

The extension writes logs beneath `@forge_crate/logs` by default. `forge_crate.log` is the aggregate stream, while domain files contain focused records for areas such as actors, banking, organizations, persistence, events, garages, lockers, notifications, and vehicle services.

```text
@forge_crate/logs/
  forge_crate.log
  actor.log
  bank.log
  events.log
  organization.log
  persistence.log
  ...
```

Each record contains an RFC 3339 UTC timestamp, severity, domain, message, and Rust source location:

```text
2026-06-20T05:12:34.567Z [DEBUG] [bank] cache pull key=76561198000000000 result=hit (arma/crate/src/persistence/repository.rs:77)
```

## Runtime Behavior

Gameplay and extension command threads never open, write, flush, or lock log files. They submit owned records with a non-blocking `try_send` into a bounded channel. The dedicated `forge-log-writer` thread owns all file handles, drains records in batches, and flushes them every 500 milliseconds.

The channel holds 8,192 records. If producers fill the channel, new records are dropped instead of stalling gameplay. The aggregate log reports the dropped-record count when the writer catches up.

Debug records cover cache pulls, cache saves, hydration, queued database operations, completed database operations, transactions, and domain-event publication. Records identify keys and counts but do not serialize full domain objects, ATM PINs, or database credentials.

## Path Overrides

Set `FORGE_SERVER_LOG_DIR` to move the complete log directory:

```powershell
$env:FORGE_SERVER_LOG_DIR="D:\ForgeLogs"
```

The existing `FORGE_SERVER_LOG` variable remains supported and controls the exact aggregate log filename. Domain logs are written beside that file:

```powershell
$env:FORGE_SERVER_LOG="D:\ForgeLogs\server-one.log"
```

When both variables are set, `FORGE_SERVER_LOG` takes precedence for the aggregate file and its parent directory becomes the domain-log directory.
