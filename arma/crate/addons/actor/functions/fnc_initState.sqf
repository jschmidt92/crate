#include "..\script_component.hpp"

/*
 * File: fnc_initState.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Initializes the local actor synchronization lifecycle.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Lifecycle hashmap [HASHMAP]
 *
 * Example:
 * call forge_crate_actor_fnc_initState;
 */

GVAR(lifecycle) = createHashMapFromArray [
    ["state", "UNINITIALIZED"],
    ["previous", ""],
    ["reason", "client initialized"],
    ["changedAt", diag_tickTime]
];

GVAR(lifecycle)
