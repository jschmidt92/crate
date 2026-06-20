#include "..\script_component.hpp"

/*
 * File: fnc_disconnectPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Persists actor disconnect state while the player object is still available. The extension event bus handles feature cleanup.
 *
 * Arguments:
 * 0: [OBJECT] - Disconnecting player
 * 1: [STRING] - Player UID fallback
 *
 * Return Value:
 * Actor disconnect payload [HASHMAP]
 *
 * Example:
 * [_player, _uid] call forge_crate_actor_fnc_disconnectPlayer;
 */

params [["_player", objNull, [objNull]], ["_uid", "", [""]]];

if (_uid isEqualTo "" && {!isNull _player}) then {
    _uid = getPlayerUID _player;
};
if (_uid isEqualTo "") exitWith { createHashMap };
if (isNull _player) exitWith { createHashMap };

private _snapshot = [_player] call FUNC(snapshotFromPlayer);
if (_snapshot isEqualTo createHashMap) exitWith { createHashMap };

["actor:disconnect", [toJSON _snapshot]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

if (_result isEqualTo "OK") exitWith { createHashMap };

private _actor = fromJSON _result;
if (_actor isEqualType createHashMap) exitWith { _actor };

createHashMap
