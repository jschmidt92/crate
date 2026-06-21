#include "..\script_component.hpp"

/*
 * File: fnc_refreshBank.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Fetches a fresh bank snapshot for the given player and pushes it
 * to their WebUI via a server-push event (no requestId).
 *
 * Arguments:
 * 0: [OBJECT] - Player object
 *
 * Return Value:
 * Push dispatched [BOOL]
 *
 * Example:
 * [player] call forge_crate_webui_fnc_refreshBank;
 */

params [["_player", objNull, [objNull]]];

diag_log format ['FORGE_BANK_REFRESH: player: %1', _player];

if (!isServer || { isNull _player }) exitWith { false };

private _uid = getPlayerUID _player;
if (_uid isEqualTo "") exitWith { false };

private _call = {
    params ["_command", "_arguments"];
    [_command, _arguments] call EFUNC(extension,call)
};

private _parseProfile = {
    params ["_result"];
    private _profile = fromJSON _result;
    if !(_profile isEqualType createHashMap) exitWith { createHashMap };
    _profile
};

// Fetch bank profile
(["bank:get", [_uid]] call _call) params ["_bankResult", "_bankSuccess"];
if (!_bankSuccess) exitWith { false };

private _bank = [_bankResult] call _parseProfile;
if (_bank isEqualTo createHashMap) exitWith { false };

// Fetch organization
(["organization:get_by_member", [_uid]] call _call) params ["_orgResult", "_orgSuccess"];
private _organization = createHashMap;
if (_orgSuccess && {_orgResult != "null"}) then {
    private _parsedOrg = fromJSON _orgResult;
    if (_parsedOrg isEqualType createHashMap) then {
        _organization = createHashMapFromArray [
            ["name", _parsedOrg getOrDefault ["name", ""]],
            ["bank", _parsedOrg getOrDefault ["bank", "0.00"]],
            ["members", _parsedOrg getOrDefault ["members", []]]
        ];
    };
};

// Build push response (no requestId so the WebUI routes it to push listeners)
private _response = createHashMapFromArray [
    ["requestId", ""],
    ["event", "bank::refresh"],
    ["ok", true],
    ["data", createHashMapFromArray [
        ["profile", _bank],
        ["organization", _organization]
    ]],
    ["error", ""]
];

[CRPC(webui,response), [_response], _player] call CFUNC(targetEvent);
true
