#include "..\script_component.hpp"

/*
 * File: fnc_disconnectPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Finalizes the player's virtual garage unlocks on disconnect, then continues the module disconnect chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Success [BOOL]
 *
 * Example:
 * [_uid] call forge_server_v_garage_fnc_disconnectPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { false };

["v_garage:disconnect", [_uid]] call EFUNC(extension,extCall) params ["", "_success"];
if !(_success) exitWith { false };

[SRPC(locker,disconnectPlayer), [_uid]] call CFUNC(localEvent);

true
