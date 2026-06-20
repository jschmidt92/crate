forge_crate_actor
==================

Actor addon for forge-crate.

The server initializes actors through the Rust extension and returns an authoritative actor record with a `created` flag. New actors are stripped and receive `CfgMission >> Actor >> loadout`. Returning actors restore their persisted loadout and supported world state. Disconnect snapshots always use the live loadout so later sessions can restore player changes.
