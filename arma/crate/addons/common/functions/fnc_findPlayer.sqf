#include "..\script_component.hpp"

/*
 * File: fnc_findPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Finds a connected player by Steam UID.
 *
 * Arguments:
 * 0: [STRING] - Steam UID
 *
 * Return Value:
 * Player object or objNull [OBJECT]
 *
 * Example:
 * [_uid] call forge_crate_common_fnc_findPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { objNull };

private _players = allPlayers;
private _index = _players findIf { getPlayerUID _x isEqualTo _uid };
if (_index < 0) exitWith { objNull };

_players select _index
