#include "..\script_component.hpp"

/*
 * File: fnc_call.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Calls the Forge server extension with request staging and response chunk assembly.
 *
 * Arguments:
 * 0: [STRING] - Extension command
 * 1: [ARRAY] - Extension arguments
 *
 * Return Value:
 * Extension result and success state [ARRAY]
 *
 * Example:
 * ["actor:init", [_snapshotJson]] call forge_crate_extension_fnc_call;
 */

params [["_command", "", [""]], ["_arguments", [], [[]]]];

if (_command isEqualTo "") exitWith { ["", false] };

private _chunkPrefix = "FORGE_TRANSPORT_CHUNK:";
private _chunkPrefixLength = count _chunkPrefix;
private _requestChunkSize = 12000;

private _callExtension = {
    params [["_extensionCommand", "", [""]], ["_extensionArguments", [], [[]]]];

    private _result = EXT callExtension [_extensionCommand, _extensionArguments];
    private _payload = _result param [0, "", [""]];
    private _extensionCode = _result param [1, -1, [0]];
    private _armaCode = _result param [2, 0, [0]];
    private _success = _extensionCode isEqualTo 0 && {_armaCode in [0, 301]} && {(_payload find "Error:") != 0};

    if !(_success) then {
        ERROR_4("Extension command %1 failed with extension code %2 arma code %3: %4",_extensionCommand,_extensionCode,_armaCode,_payload);
    };

    [_payload, _success]
};

private _directCommands = ["version", "status", "database_status", "config_path", "log_path"];
if (_command in _directCommands || {(_command find "refuel:") == 0} || {(_command find "transport:") == 0}) exitWith {
    [_command, _arguments] call _callExtension
};

private _stringArguments = _arguments apply {
    if (_x isEqualType "") then {
        _x
    } else {
        if (_x isEqualType true) then {
            ["false", "true"] select _x
        } else {
            str _x
        };
    };
};
private _argumentsJson = toJSON _stringArguments;

private _transportCommand = "transport:invoke";
private _transportArguments = [_command, _argumentsJson];

if ((count _argumentsJson) > _requestChunkSize) then {
    ["stage", _command, _argumentsJson, _requestChunkSize, _callExtension] call FUNC(chunks) params [
        "_stagedCommand",
        "_stagedArguments",
        "_stageSuccess"
    ];
    if (!_stageSuccess) exitWith { ["Error: Failed to stage chunked extension request", false] };
    _transportCommand = _stagedCommand;
    _transportArguments = _stagedArguments;
};

[_transportCommand, _transportArguments] call _callExtension params ["_payload", "_success"];

["assemble", _payload, _success, _chunkPrefix, _chunkPrefixLength, _callExtension] call FUNC(chunks)
