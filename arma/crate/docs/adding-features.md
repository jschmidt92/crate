# Adding Rust Server Features

This guide is the practical checklist for adding a new Rust server feature.

## Before You Start

Identify what kind of change you are making:

- Domain rule: belongs in `forge-lib`.
- Server workflow: belongs in `arma/crate/src/features`.
- Arma command surface: belongs in `arma/crate/src/<command_group>.rs` and possibly `command.rs`.
- Persistence mechanism: belongs in `arma/crate/src/persistence`.
- Cross-cutting side effect: usually belongs behind a domain event handler.

## Recommended Flow

1. Add or update domain models.
2. Add or update repository traits if new storage access is needed.
3. Add or update service methods for validation and core rules.
4. Add a server feature slice for workflow orchestration.
5. Publish domain events through `EventPublisher` if needed.
6. Add durable audit/notification side effects if needed.
7. Add or update command routes.
8. Add tests.
9. Run `cargo fmt` and `cargo test`.

## Domain Models

Add shared data types under:

```text
lib/src/models
```

Use domain-specific files when possible. For example:

- Organization data: `organization.rs`
- Organization events: `organization_event.rs`
- Notifications/audit: `notification.rs`

Export new public models from:

```text
lib/src/models/mod.rs
```

## Repository Traits

Repository traits live under:

```text
lib/src/repositories
```

Add repository methods when services need new storage operations. Keep traits focused on domain storage needs, not database-specific details.

In-memory repositories should be updated alongside the trait so service tests can exercise the behavior without SurrealDB.

Server cached repository implementations live in:

```text
arma/crate/src/persistence/repository.rs
```

## Services

Services live under:

```text
lib/src/services
```

Services should:

- validate inputs.
- enforce business rules.
- mutate domain models.
- call repository traits.
- return domain results.

Services should not:

- know about Arma commands.
- know about SurrealDB.
- directly publish server events.
- serialize responses.

## Feature Slices

Server feature workflows live under:

```text
arma/crate/src/features
```

Organization is the current example:

```text
features/organization/
  create.rs
  invite.rs
  membership.rs
  payday.rs
  query.rs
  mod.rs
```

A feature slice should orchestrate one use case or a small use-case group. It can call services, publish events, and compose persistence helpers through interfaces.

Good slice responsibilities:

- call the domain service.
- call `EventPublisher`.
- call a workflow-specific port, such as `PaydayApplier`.
- return typed results to command modules.

Avoid putting JSON serialization or Arma-specific argument parsing in feature slices.

## Commands

Arma command groups live in files such as:

```text
arma/crate/src/organization.rs
arma/crate/src/actor.rs
arma/crate/src/bank.rs
```

Command functions should:

- parse string arguments.
- parse JSON payloads when needed.
- call a feature workflow.
- serialize the result.
- log failures.

Transport route strings are listed in:

```text
arma/crate/src/command.rs
```

When adding a command, update both the arma-rs group and transport dispatcher if the command should be available through both paths.

## Events

Use domain events when other parts of the system need to react after a successful action.

Add events by updating:

```text
lib/src/models/<domain>_event.rs
lib/src/models/domain_event.rs
lib/src/models/mod.rs
```

Publish events from the feature workflow through `EventPublisher`.

Do not call persistence directly to create audit rows or notifications. Add durable side effects to:

```text
arma/crate/src/persistence/durable_events.rs
```

## Persistence

Persistence code lives under:

```text
arma/crate/src/persistence
```

Use:

- `repository.rs` for cached repository implementations.
- `service.rs` for the background persistence worker.
- `surreal.rs` for SurrealDB connection and query execution.
- `model.rs` for queued write operation models.
- feature-specific files, such as `payday.rs`, for special transactional write flows.

The server keeps hot in-memory repositories and queues writes to the persistence worker. Gameplay command responses do not wait for SurrealDB.

For multi-record money movement, use a batch transaction pattern like `persistence/payday.rs`.

## Testing

Minimum expectations:

- Service behavior should have unit tests in `forge-lib`.
- New validation rules should have focused tests.
- Repository trait changes should be covered by in-memory repositories.
- Workflow-level behavior should be tested either in service tests or future feature tests.

Run:

```powershell
cargo fmt
cargo test
```

## Checklist

Use this checklist before calling a feature done:

- Models are in the right domain file.
- New models are exported from `models/mod.rs`.
- Service rules are covered by tests.
- Commands are thin and do not contain core business logic.
- Events are published through `EventPublisher`.
- Durable side effects are in `persistence/durable_events.rs`.
- Persistence changes update hydration if a new table is introduced.
- `docs/persistence.md` lists new persistent tables.
- `cargo fmt` passes.
- `cargo test` passes.
