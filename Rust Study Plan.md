# Rust Study Plan for Understanding the Forge Codebase

## Goal

Use this plan to understand the Rust concepts, architecture, and use cases in this codebase:

* `G:/forge/lib`: domain models, services, repository traits, events.
* `G:/forge/arma/crate`: Arma extension boundary, command routing, persistence, async runtime.

By the end of this plan, you should be able to explain and modify one full use case, such as player initialization or bank transfer, from Arma command input all the way through domain logic, events, repository updates, and persistence.

---

# Phase 0: Setup, Tooling, and Workflow

Before studying Rust deeply, get comfortable with the tools you will use every day.

## Install and configure

Use:

* [rustup](https://rustup.rs/)
* VS Code or your preferred editor
* `rust-analyzer`
* Git
* GitHub Desktop or command-line Git, whichever you prefer

## Learn these commands

Practice using:

```bash
cargo build
cargo check
cargo test
cargo fmt
cargo clippy
cargo run
cargo clean
```

## What to learn

* How to create and open a Rust project.
* How `Cargo.toml` works at a basic level.
* How to run tests.
* How to format code with `cargo fmt`.
* How to use `cargo clippy` to find common mistakes.
* How to read compiler errors without panicking.
* How Rust compiler messages often explain the fix.

## Practice

Create a small throwaway Rust project and intentionally make mistakes:

* Borrow something incorrectly.
* Return the wrong type.
* Forget a semicolon.
* Use a moved value.
* Run `cargo check` and read the error carefully.

Do not rush this phase. Rust’s compiler is one of the best learning tools you have.

---

# Phase 1: Rust Foundations

Use these first:

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
  Main book. Use this as the primary learning resource.

* [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
  Short runnable examples for concepts like structs, enums, traits, modules, error handling, testing, and generics.

* [Rustlings](https://github.com/rust-lang/rustlings)
  Small exercises intended to be done alongside the Rust Book.

## Study these topics in order

1. Variables and mutability.
2. Functions.
3. Ownership.
4. Borrowing and references.
5. Basic lifetimes.
6. Structs.
7. Enums.
8. Pattern matching.
9. `Option`.
10. `Result`.
11. Error propagation with `?`.
12. Traits.
13. Generic bounds.
14. Modules, crates, and visibility.
15. Unit tests.

## Also learn these early

These are very common in real Rust code:

* `String` vs `&str`
* `Vec<T>`
* `HashMap<K, V>`
* Iterators
* `map`
* `filter`
* `collect`
* `for` loops
* `match`
* `if let`
* `while let`

## What to focus on

Rust is strict because it wants memory safety without a garbage collector. Do not try to memorize everything right away. Focus on understanding:

* Who owns this value?
* Is this value borrowed?
* Is it borrowed mutably or immutably?
* How long does the borrowed value need to live?
* What happens when this function returns?

## Then revisit these repo files

* `lib/src/models/bank.rs`
* `lib/src/shared/error.rs`
* `lib/src/services/bank.rs`

## Practice

For each file:

1. Identify the structs.
2. Identify the enums.
3. Identify the public methods.
4. Find where `Result` is used.
5. Find where custom errors are returned.
6. Find one test and explain what behavior it proves.

---

# Phase 2: Cargo, Workspaces, and Crate Structure

Use:

* [The Cargo Book](https://doc.rust-lang.org/cargo/)
* [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)

## Focus on

1. `Cargo.toml`
2. Workspace members
3. Path dependencies
4. Feature flags
5. `cargo test`
6. `cargo check`
7. `cargo fmt`
8. `cargo clippy`
9. Package selection
10. Crate types, especially `lib` and `cdylib`

## Apply it to

* root `Cargo.toml`
* `lib/Cargo.toml`
* `arma/crate/Cargo.toml`

## Key concept

This repo separates core logic from extension/runtime code.

* `forge-lib` contains reusable domain logic.
* `forge-crate` acts as the adapter around that logic for Arma.

This separation is important because the business logic can be tested and reused without needing to run inside Arma.

## Practice

Run these from the workspace root:

```bash
cargo check
cargo test
cargo fmt
cargo clippy
```

Then try package-specific commands:

```bash
cargo test -p forge-lib
cargo check -p forge-crate
```

---

# Phase 3: Domain Modeling and Services

Use:

* Rust Book chapters on structs, enums, traits, error handling, and tests.
* Rust By Example sections on custom types, conversion, traits, and error handling.

## Study in the repo

1. `lib/src/models/bank.rs`
2. `lib/src/models/actor.rs`
3. `lib/src/models/organization.rs`
4. `lib/src/services/bank.rs`
5. `lib/src/services/actor.rs`
6. `lib/src/services/organization.rs`

## What to learn

* Why `Money` is a type instead of raw `i64`, `u64`, or `f64`.
* How domain models protect business rules.
* How view models like `PlayerBankProfileView` protect serialized output.
* How services enforce business rules.
* How tests document expected behavior.
* Where errors are created.
* Where errors are returned.
* Which methods change state and which only read state.

## Recommended reading habit

For every feature:

1. Read the tests first.
2. Read the public service method.
3. Read the model methods called by that service.
4. Read the repository methods used by the service.
5. Run the test.
6. Change one assertion and observe the failure.
7. Undo the change.

## Practice

Pick one test in:

* `lib/src/services/bank.rs`

Then:

1. Read the test name.
2. Explain the expected behavior in plain English.
3. Read the service method it tests.
4. Read the model methods used by that service.
5. Trace every possible `Result` or error path.
6. Run only that test if possible.

---

# Phase 4: Error Handling and Domain Errors

Use:

* Rust Book error handling chapter
* Rust By Example error handling section

## Study

* `lib/src/shared/error.rs`
* Service files that return custom errors
* Command or response files that convert errors into output

## What to learn

* Difference between `Option` and `Result`.
* When to use `unwrap` in tests.
* Why production code should usually avoid `unwrap`.
* How custom error types describe business failures.
* How errors move from domain logic to command output.
* Whether an error came from parsing, validation, persistence, or business rules.

## Practice

Pick one custom error and trace it:

1. Where is the error defined?
2. Where is it created?
3. What condition causes it?
4. Which service returns it?
5. How does it eventually become a response?
6. Is it a domain error, command error, persistence error, or validation error?

---

# Phase 5: Repository Pattern and In-Memory Storage

Use Rust Book material on traits and generics, then read:

* `lib/src/repositories/bank.rs`
* `lib/src/repositories/actor.rs`
* `lib/src/repositories/organization.rs`

Supplement with standard library docs:

* [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
* [LazyLock](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
* [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
* [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

## Prep topics

Before going too deep, learn the basic purpose of:

* `Box<T>`
* `Arc<T>`
* `Mutex<T>`
* `RwLock<T>`
* Interior mutability
* Shared ownership
* Shared mutable state

## What to learn

* Repository traits define storage behavior.
* In-memory repositories support tests and local operation.
* `Arc<RwLock<_>>` allows shared mutable state safely.
* Services depend on traits, not concrete databases.
* Traits make it possible to swap storage implementations.
* The service layer should not need to know whether data is in memory or in a database.

## Practice

Pick one repository trait and answer:

1. What behavior does this trait promise?
2. What data does it store?
3. Which services depend on it?
4. Is there an in-memory implementation?
5. How would a database-backed implementation satisfy the same trait?

---

# Phase 6: Serialization and Command Boundaries

Use:

* [Serde overview](https://serde.rs/)

## Study

* `lib/src/models/*.rs`
* `arma/crate/src/response.rs`
* `arma/crate/src/transport.rs`
* `arma/crate/src/actor.rs`
* `arma/crate/src/bank.rs`
* `arma/crate/src/command.rs`

## What to learn

* `Serialize`
* `Deserialize`
* JSON as the boundary between Arma and Rust
* Why command handlers parse strings
* Why command handlers return strings
* How command output is shaped
* How large responses are chunked in `transport.rs`
* Where domain types become response types

## Practice

Trace this command:

```text
bank:deposit
```

Follow it from:

1. `arma/crate/src/command.rs`
2. Bank command module
3. Request parsing
4. Service method
5. Domain model changes
6. Repository update
7. Response creation
8. String returned back to Arma

Write down every type conversion that happens along the way.

---

# Phase 7: Debugging, Logging, and Runtime Flow

This phase is about learning how to see what the code is doing.

## Learn these tools and concepts

* `println!`
* `dbg!`
* compiler warnings
* `cargo test -- --nocapture`
* logging or tracing, depending on what the repo uses
* reading stack traces or panic output
* tracing command flow through multiple files

## What to learn

* Where to add temporary debug output.
* How to inspect values while learning.
* How to debug failing tests.
* How to follow async or background behavior.
* How to avoid leaving messy debug output in committed code.

## Practice

Pick one command flow and add temporary debug output:

1. Print when the command is received.
2. Print after request parsing.
3. Print before the service call.
4. Print after the service call.
5. Print the response before returning it.

Then remove the debug output once you understand the flow.

---

# Phase 8: Events and Domain Side Effects

Use Rust Book material on enums, traits, and pattern matching.

## Study

* `lib/src/models/domain_event.rs`
* `lib/src/events/bus.rs`
* `lib/src/events/handlers/*`
* `arma/crate/src/events.rs`

## What to learn

* `DomainEvent` models important business facts.
* `EventBus` dispatches events to handlers.
* Events allow one action to trigger related side effects.
* `ActorCreated` provisions related resources.
* `ActorDisconnected` triggers cleanup or persistence behavior.
* Events help avoid putting every side effect directly inside one service method.

## Practice

Trace player initialization:

```text
actor:init -> ActorService::init_or_create -> ActorCreated -> event handlers
```

Answer:

1. What starts the flow?
2. What service method is called?
3. What event is created?
4. Which handlers respond to the event?
5. What side effects happen?
6. What gets persisted or queued?

---

# Phase 9: Async Rust and Persistence

Use:

* [Tokio tutorial](https://tokio.rs/tokio/tutorial)
* [SurrealDB Rust crate docs](https://docs.rs/surrealdb/latest/surrealdb/)

## Study

* `arma/crate/src/lib.rs`
* `arma/crate/src/persistence/mod.rs`
* `arma/crate/src/persistence/repository.rs`
* `arma/crate/src/persistence/service.rs`
* `arma/crate/src/persistence/surreal.rs`

## What to learn

* Tokio runtime creation
* `async` and `await`
* Background workers
* `mpsc` channels
* Retry loops
* Cache hydration
* Write-behind persistence
* How persistence errors are handled
* How in-memory state and database state stay coordinated

## Key idea

The game-facing code stays fast by writing to memory first, then queueing database writes.

That means Arma does not need to wait on every database operation before continuing.

## Practice

Trace one persistence flow:

1. What triggers persistence?
2. Is the write immediate or queued?
3. What data is sent to the persistence layer?
4. What happens if the database write fails?
5. Is there a retry?
6. How does the in-memory cache get hydrated?
7. What happens on startup?
8. What happens on shutdown or disconnect?

---

# Phase 10: Arma Extension Layer

Use:

* [arma-rs docs](https://docs.rs/arma-rs/latest/arma_rs/)

## Study

* `arma/crate/src/lib.rs`
* `arma/crate/src/command.rs`
* `arma/crate/src/transport.rs`
* One feature group, such as:

  * `arma/crate/src/actor.rs`
  * `arma/crate/src/bank.rs`

## What to learn

* `#[arma]`
* `Extension::build()`
* `Group`
* Command naming like `actor:init`
* `cdylib`
* Why the extension returns strings
* How Rust functions are exposed to Arma
* How command routing keeps the boundary organized

## Practice

Pick one feature group and explain:

1. What commands does it expose?
2. What input does each command expect?
3. What service does each command call?
4. What response does each command return?
5. What errors can happen?
6. How would you add one new command?

---

# Phase 11: Final Capstone

The final goal is to make one small change end-to-end.

## Pick one small feature

Examples:

* Add a new bank query command.
* Add a new actor lookup command.
* Add a new organization query command.
* Add a small validation rule.
* Add a new service method.
* Add a new response view model.

## Required steps

1. Write or update a test first.
2. Update the domain model if needed.
3. Update the service.
4. Update the repository trait if needed.
5. Update the in-memory repository implementation if needed.
6. Update the command adapter.
7. Update response serialization.
8. Run `cargo fmt`.
9. Run `cargo clippy`.
10. Run `cargo test`.
11. Manually trace the full command flow.
12. Document what changed.

## Success criteria

You are done when you can explain:

* What command starts the flow.
* What data enters Rust.
* How the data is parsed.
* What service method is called.
* What business rule is applied.
* What repository method is used.
* Whether any event is emitted.
* Whether anything is persisted.
* What response is returned to Arma.
* What tests prove the behavior.

---

# Suggested Books

Use these after the Rust Book, not before:

* *Programming Rust*, O’Reilly
  Best for deeper systems-level understanding.

* *Rust for Rustaceans*, No Starch
  Best after you can already read Rust code.

* *Zero to Production in Rust*, Luca Palmieri
  Useful for service architecture, testing, async, and application structure.

---

# Best Weekly Schedule

This version assumes roughly 8 to 10 hours per week.

## Week 1: Setup and Rust basics

* Install Rust tooling.
* Install editor support.
* Learn `cargo check`, `cargo build`, `cargo test`, `cargo fmt`, and `cargo clippy`.
* Rust Book chapters 1-4.
* Rustlings basics through ownership.

Focus:

* Variables
* Functions
* Ownership
* Borrowing
* References
* Reading compiler errors

## Week 2: Core Rust types

* Rust Book chapters 5-8.
* Rustlings exercises for structs, enums, and collections.

Focus:

* Structs
* Enums
* Pattern matching
* `String` vs `&str`
* `Vec`
* `HashMap`
* Basic iterators

## Week 3: Traits, errors, tests, and lifetimes

Focus:

* Traits
* Generics
* `Option`
* `Result`
* Error propagation
* Basic lifetimes
* Unit tests

Repo focus:

* `lib/src/shared/error.rs`
* `lib/src/models/bank.rs`
* `lib/src/services/bank.rs`

## Week 4: Cargo and workspace structure

Focus:

* Cargo workspaces
* Crates
* Path dependencies
* Feature flags
* Package-specific commands
* `lib` vs `cdylib`

Repo focus:

* root `Cargo.toml`
* `lib/Cargo.toml`
* `arma/crate/Cargo.toml`

## Week 5: Models and services

Focus:

* Domain modeling
* Business rules
* Service methods
* View models
* Reading tests first

Repo focus:

* `lib/src/models/bank.rs`
* `lib/src/models/actor.rs`
* `lib/src/models/organization.rs`
* `lib/src/services/bank.rs`
* `lib/src/services/actor.rs`
* `lib/src/services/organization.rs`

## Week 6: Repositories and shared state

Focus:

* Repository pattern
* Traits as abstractions
* In-memory storage
* `Arc`
* `RwLock`
* `Mutex`
* `LazyLock`
* Interior mutability

Repo focus:

* `lib/src/repositories/bank.rs`
* `lib/src/repositories/actor.rs`
* `lib/src/repositories/organization.rs`

## Week 7: Serialization, command boundaries, and debugging

Focus:

* Serde
* JSON
* Request parsing
* Response formatting
* Command routing
* `println!`
* `dbg!`
* `cargo test -- --nocapture`

Repo focus:

* `arma/crate/src/command.rs`
* `arma/crate/src/response.rs`
* `arma/crate/src/transport.rs`
* `arma/crate/src/actor.rs`
* `arma/crate/src/bank.rs`

Trace:

```text
bank:deposit
```

## Week 8: Events and cross-feature workflows

Focus:

* Domain events
* Event bus
* Event handlers
* Side effects
* Player initialization flow

Repo focus:

* `lib/src/models/domain_event.rs`
* `lib/src/events/bus.rs`
* `lib/src/events/handlers/*`
* `arma/crate/src/events.rs`

Trace:

```text
actor:init -> ActorService::init_or_create -> ActorCreated -> event handlers
```

## Week 9: Async, persistence, and SurrealDB

Focus:

* Tokio runtime
* `async`
* `await`
* Background workers
* `mpsc`
* Retry loops
* Cache hydration
* Write-behind persistence
* SurrealDB integration

Repo focus:

* `arma/crate/src/lib.rs`
* `arma/crate/src/persistence/mod.rs`
* `arma/crate/src/persistence/repository.rs`
* `arma/crate/src/persistence/service.rs`
* `arma/crate/src/persistence/surreal.rs`

## Week 10: Arma extension layer and capstone

Focus:

* `arma-rs`
* `#[arma]`
* `Extension::build()`
* `Group`
* `cdylib`
* Command naming
* String-based command boundary

Repo focus:

* `arma/crate/src/lib.rs`
* `arma/crate/src/command.rs`
* `arma/crate/src/transport.rs`
* One full feature group

Final task:

Add or modify one small feature end-to-end, including tests, service logic, command adapter, response output, formatting, linting, and documentation.

---

# Ongoing Study Rules

Use these rules throughout the plan.

## Rule 1: Read tests first

Tests often explain the intended behavior better than the implementation.

## Rule 2: Trace one full flow at a time

Do not jump randomly through the codebase. Pick one command or use case and follow it from input to output.

## Rule 3: Write notes in plain English

For each feature, write:

* What starts this flow?
* What data comes in?
* What service is called?
* What changes?
* What gets saved?
* What response comes back?

## Rule 4: Run the code often

Use:

```bash
cargo check
cargo test
cargo fmt
cargo clippy
```

Do not wait until the end of a study session to run the tools.

## Rule 5: Prefer small changes

When practicing, change one thing at a time.

Good practice changes:

* Add one test.
* Add one validation rule.
* Add one command.
* Add one response field.
* Rename one method after understanding it.
* Refactor one small duplicate section.

Avoid large rewrites until the full flow makes sense.

---

# Final Outcome

After completing this plan, you should be able to:

* Read Rust service code without getting lost.
* Understand ownership and borrowing in normal application code.
* Understand how this repo separates domain logic from runtime/extension logic.
* Explain why repository traits are used.
* Follow command input from Arma into Rust.
* Understand how JSON responses are created.
* Understand how domain events trigger side effects.
* Understand the async persistence queue.
* Run tests and interpret failures.
* Make one small feature change safely.
