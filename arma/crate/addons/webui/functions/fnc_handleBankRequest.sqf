#include "..\script_component.hpp"

params [
    ["_player", objNull, [objNull]],
    ["_requestId", "", [""]],
    ["_event", "", [""]],
    ["_data", createHashMap, [createHashMap]]
];

diag_log format ['FORGE_BANK_REQUEST: player: %1', _player];
diag_log format ['FORGE_BANK_REQUEST: requestId: %1', _requestId];
diag_log format ['FORGE_BANK_REQUEST: event: %1, data: %2', _event, _data];

if (!isServer || { isNull _player } || { _requestId isEqualTo "" }) exitWith { false };

private _uid = getPlayerUID _player;
if (_uid isEqualTo "") exitWith { false };

private _response = createHashMapFromArray [
    ["requestId", _requestId],
    ["event", _event],
    ["ok", false],
    ["data", createHashMap],
    ["error", "Unknown bank request"]
];

private _call = {
    params ["_command", "_arguments"];
    [_command, _arguments] call EFUNC(extension,extCall)
};

private _parseProfile = {
    params ["_result"];
    private _profile = fromJSON _result;
    if !(_profile isEqualType createHashMap) exitWith { createHashMap };
    _profile
};

private _result = "";
private _success = false;

switch (_event) do {
    case "bank::load": {
        (["bank:get", [_uid]] call _call) params ["_bankResult", "_bankSuccess"];
        if (_bankSuccess) then {
            private _bank = [_bankResult] call _parseProfile;
            if (_bank isNotEqualTo createHashMap) then {
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

                _response set ["ok", true];
                _response set ["data", createHashMapFromArray [
                    ["profile", _bank],
                    ["organization", _organization]
                ]];
                _response set ["error", ""];
            } else {
                _response set ["error", "Bank profile response was invalid"];
            };
        } else {
            _response set ["error", _bankResult];
        };
    };
    case "bank::deposit": {
        private _res = ["bank:deposit", [_uid, _data getOrDefault ["amount", 0]]] call _call;
        _result = _res select 0;
        _success = _res select 1;
    };
    case "bank::withdraw": {
        private _res = ["bank:withdraw", [_uid, _data getOrDefault ["amount", 0]]] call _call;
        _result = _res select 0;
        _success = _res select 1;
    };
    case "bank::transfer": {
        private _res = ["bank:transfer", [
            _uid,
            _data getOrDefault ["target", ""],
            _data getOrDefault ["amount", 0]
        ]] call _call;
        _result = _res select 0;
        _success = _res select 1;

        if (_success) then {
            private _transfer = fromJSON _result;
            if (_transfer isEqualType createHashMap) then {
                _result = toJSON (_transfer getOrDefault ["from", createHashMap]);
            } else {
                _success = false;
                _result = "Transfer response was invalid";
            };
        };
    };
    case "bank::submit_earnings": {
        private _res = ["bank:submit_earnings", [_uid]] call _call;
        _result = _res select 0;
        _success = _res select 1;
    };
    case "bank::change_pin": {
        private _res = ["bank:change_pin", [
            _uid,
            _data getOrDefault ["currentPin", ""],
            _data getOrDefault ["newPin", ""]
        ]] call _call;
        _result = _res select 0;
        _success = _res select 1;
    };
};

if (_event != "bank::load" && {_success}) then {
    private _profile = [_result] call _parseProfile;
    if (_profile isNotEqualTo createHashMap) then {
        _response set ["ok", true];
        _response set ["data", createHashMapFromArray [["profile", _profile]]];
        _response set ["error", ""];
    } else {
        _response set ["error", "Bank profile response was invalid"];
    };
} else {
    if (_event != "bank::load" && {!_success} && {_result != ""}) then {
        _response set ["error", _result];
    };
};

[CRPC(webui,bankResponse), [_response], _player] call CFUNC(targetEvent);
true
