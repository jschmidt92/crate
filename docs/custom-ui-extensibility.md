# Custom UI Extensibility

Read [WebUI and Browser Bridge](webui.md) first. This guide focuses on extending or replacing the presentation layer.

## Supported Baseline

The current implementation supports:

- one `Forge_WebUI_Display`.
- one browser control.
- one `JSDialog` route handler.
- `ui::close`.
- `bank::*` request routing.
- correlated browser promises.
- server-pushed browser events.

Other namespaces require code in both SQF and the WebUI.

## Reusing the Bridge

Use:

```typescript
requestFromArma<Result>("feature::action", {
    value: "example"
});
```

The bridge creates a request ID, stores a pending promise, and rejects after ten seconds.

Server responses must use:

```typescript
type ForgeHostResponse<T> = {
    requestId: string;
    event: string;
    ok: boolean;
    data: T;
    error: string;
};
```

## Adding a Namespace

Example: `market::quote`.

### Browser

```typescript
const quote = await requestFromArma<Quote>("market::quote", {
    classname
});
```

### Client Router

Add a `market` branch in:

```text
arma/crate/addons/webui/functions/fnc_route.sqf
```

Route the request to a server CBA event with:

- player object.
- request ID.
- event name.
- data hashmap.

### Server Handler

The server handler must:

1. validate player and UID.
2. authorize the operation.
3. call the Rust extension.
4. build the standard response envelope.
5. target the requesting client.

### Rust

Add a thin command and a feature workflow. If the new domain spends money, compose `BankService` in Rust so the invariant is enforced in one authoritative workflow.

## Current vs Proposed Events

Do not assume events such as:

```text
forge_crate_bank_deductRequested
forge_crate_bank_deductSucceeded
forge_crate_bank_deductFailed
```

exist. They are not implemented by the current bank addon.

For a new purchase workflow, prefer a Rust application feature that composes market and bank services and performs required transactional persistence. CBA event choreography is appropriate for SQF engine-side follow-up, not for replacing atomic server invariants.

## Replacing the UI

You may:

- replace the Preact component tree.
- add feature routes within the same bundle.
- package a separate local HTML application and create a custom display.

Preserve these host requirements:

- load packaged assets with `A3API.RequestFile`.
- send host messages with `A3API.SendAlert`.
- expose `window.forgeHostReceive`.
- use `A3API.NavigateTo` for Arma browser navigation.

## Compatibility

- Guard `localStorage`, clipboard, and other origin-sensitive APIs.
- Prefer ASCII for source text that must survive Arma's file bridge.
- Test CSS in the embedded browser, not only desktop Chromium.
- Provide a static loader before the application bundle mounts.
- Keep runtime errors visible through the development console during testing.

## Theme Behavior

Dark is the default. Theme switching works for the current display session. Normal browsers persist the choice; Arma may reopen in dark mode because its opaque origin can deny persistent storage.
