# Extension Command Reference

SQF normally calls commands through:

```sqf
["route", [_arg1, _arg2]] call forge_crate_extension_fnc_call
```

All transport arguments are converted to strings. Complex objects are serialized as JSON.

Success returns:

```sqf
[_payload, true]
```

Failure returns an `Error:` payload and `false`.

## System Commands

| Command | Arguments | Result |
| --- | --- | --- |
| `version` | none | Extension version string. |
| `status` | none | Basic runtime status. |
| `database_status` | none | `disabled`, `disconnected`, `hydrating`, or `ready`, plus queue metrics. |
| `database_ready` | none | `true` when repositories can safely serve actor initialization. Disabled persistence is immediately ready. |
| `config_path` | none | Active TOML path. |
| `log_path` | none | Aggregate log path. |

## Actor

| Command | Arguments | Result |
| --- | --- | --- |
| `actor:init` | actor snapshot JSON | `{ actor, created }` |
| `actor:save` | actor snapshot JSON | Actor JSON |
| `actor:disconnect` | actor snapshot JSON | Actor JSON and `ActorDisconnected` publication |
| `actor:get` | UID | Actor JSON or `null` |
| `actor:delete` | UID | `OK` |

## Bank

| Command | Arguments | Result |
| --- | --- | --- |
| `bank:init` | UID, starting cash, starting bank | Bank profile view |
| `bank:get` | UID | Bank profile view or `null` |
| `bank:deposit` | UID, amount | Updated profile |
| `bank:withdraw` | UID, amount | Updated profile |
| `bank:transfer` | source UID, target UID, amount | Transfer result containing source and target profiles |
| `bank:add_earnings` | UID, amount | Updated profile |
| `bank:submit_earnings` | UID | Updated profile |
| `bank:change_pin` | UID, current PIN, new PIN | Updated profile |

Money values are serialized as decimal strings in public views.

## Organization

| Command | Arguments |
| --- | --- |
| `organization:create_default` | starting bank, virtual-garage JSON, virtual-locker JSON |
| `organization:create_player` | ID, name, CEO UID |
| `organization:disband` | organization ID, actor UID |
| `organization:create_invite` | inviter UID, organization ID, invitee UID |
| `organization:accept_invite` | invitee UID, invite ID |
| `organization:decline_invite` | invitee UID, invite ID |
| `organization:leave_member` | organization ID, UID |
| `organization:kick_member` | organization ID, actor UID, kicked UID |
| `organization:add_member` | organization ID, UID |
| `organization:get` | organization ID |
| `organization:get_by_member` | UID |
| `organization:issue_payday` | issuer UID, organization ID, amount, default-CEO-slot boolean |

## Garage and Storage

Physical garage:

- `garage:init`
- `garage:get`
- `garage:save`
- `garage:delete`

Physical locker:

- `locker:init`
- `locker:get`
- `locker:save`
- `locker:commit`
- `locker:delete`

Virtual garage:

- `v_garage:init`
- `v_garage:get`
- `v_garage:save`
- `v_garage:delete`

Virtual locker:

- `v_locker:init`
- `v_locker:get`
- `v_locker:save`
- `v_locker:delete`

Save and commit commands use JSON records. Init commands use a UID and, for virtual features, mission-configured unlock JSON.

## Notifications

| Command | Arguments | Result |
| --- | --- | --- |
| `notification:list` | UID | All notifications |
| `notification:unread` | UID | Unread notifications |
| `notification:mark_read` | UID, notification UUID | Updated notification |
| `notification:mark_all_read` | UID | Updated notifications |

## Gameplay Services

| Command | Arguments |
| --- | --- |
| `repair:quote` | damage ratio, full repair price |
| `repair:complete` | UID, vehicle ID, damage ratio, full repair price |
| `rearm:quote` | unit count, unit price |
| `rearm:complete` | UID, vehicle ID, unit count, unit price |
| `refuel:quote` | liters, fuel type, configured price |
| `refuel:complete` | UID, vehicle ID, liters, fuel type, configured price |
| `medical:respawn` | UID, respawn price |
| `medical:heal` | UID, full heal price |

Refuel also exposes session commands:

- `refuel:started`
- `refuel:tick`
- `refuel:stopped`
- `refuel:price`

## Transport

Large JSON requests and responses are chunked automatically by the SQF extension wrapper.

Internal transport commands:

- `transport:invoke`
- `transport:invoke_stored`
- `transport:request:append`
- `transport:response:get`
- `transport:response:clear`

Feature code should call `forge_crate_extension_fnc_call` rather than invoking these directly.
