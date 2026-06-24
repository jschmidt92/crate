# Feature Inventory

This page summarizes implemented surfaces and maturity.

| Feature | Rust domain | Rust application | SQF integration | WebUI | Persistence |
| --- | --- | --- | --- | --- | --- |
| Actor | Complete | Complete | Initialization, restore, save, disconnect | None | `actor` |
| Bank | Complete | Complete | Init and bank terminals | Complete | `bank` |
| Organization | Complete | Complete | Default org, membership lookup, payday helper | Snapshot only in bank UI | `organization`, `organization_invite` |
| Notifications | Complete | Complete | Query and system-chat delivery | Not yet an inbox | `notification`, audits/events |
| Physical Garage | Model/service/storage | Lifecycle/query/storage | Initialization chain | None | `garage` |
| Physical Locker | Complete transfer model | Lifecycle/query/storage/transfer | Networked proxy and actor-save handshake | None | `locker` |
| Virtual Garage | Unlock model/service | Lifecycle/query/storage | CBA enable/persistence controls | None | `v_garage` |
| Virtual Locker | Unlock model/service | Lifecycle/query/storage | ACE Arsenal and actor-save handshake | None | `v_locker` |
| Repair | Quote/receipt/service | Complete | Command surface available | None | Bank changes |
| Rearm | Quote/receipt/service | Complete | Command surface available | None | Bank changes |
| Refuel | Session and service models | Complete | Economy event hooks | None | Bank changes |
| Medical | Receipt/service | Complete | Command surface available | None | Bank changes |
| Commander | SQF only | Not applicable | Dynamic AI virtualization | None | None |
| Economy addon | Reserved | Refuel routes live in Rust | Refuel CBA hook adapter | None | Indirect |

## Important Boundaries

- “Complete” means the current workflow and tests exist; it does not imply every planned gameplay UI exists.
- Garage currently has persistence/application building blocks but no documented Eden interaction equivalent to bank or locker.
- Organization player-management operations exist in Rust, while the current WebUI only displays a read-only organization snapshot.
- Notifications have durable storage and SQF delivery but no browser inbox.
- Three.js source is retained for experimentation but is not part of the active bank application.
