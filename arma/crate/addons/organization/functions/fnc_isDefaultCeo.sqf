#include "..\script_component.hpp"

/*
 * File: fnc_isDefaultCeo.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Checks whether a connected player is occupying the mission-configured default organization CEO unit.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Player is in the default organization CEO slot [BOOLEAN]
 *
 * Example:
 * [_uid] call forge_crate_organization_fnc_isDefaultCeo;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { false };

private _player = [_uid] call EFUNC(common,findPlayer);
if (isNull _player) exitWith { false };

vehicleVarName _player isEqualTo "ceo"
