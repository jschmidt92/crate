#include "..\script_component.hpp"

/*
 * File: fnc_disconnectPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Finalizes the player's virtual locker unlocks on disconnect and completes the module disconnect chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Success [BOOL]
 *
 * Example:
 * [_uid] call forge_server_v_locker_fnc_disconnectPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { false };

["v_locker:disconnect", [_uid]] call EFUNC(extension,extCall) params ["", "_success"];

_success
