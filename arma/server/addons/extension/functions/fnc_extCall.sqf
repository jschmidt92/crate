#include "..\script_component.hpp"

/*
 * File: fnc_extCall.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Calls the Forge server extension and returns the raw result.
 * This is the extension boundary for SQF. Chunked response handling should be
 * added here so callers keep the same [result, success] contract.
 *
 * Arguments:
 * 0: [STRING] - Extension command
 * 1: [ARRAY] - Extension arguments
 *
 * Return Value:
 * Extension result and success state [ARRAY]
 *
 * Example:
 * ["actor:init", [_snapshotJson]] call forge_server_extension_fnc_extCall;
 */

params [["_command", "", [""]], ["_arguments", [], [[]]]];

if (_command isEqualTo "") exitWith { ["", false] };

private _result = EXT callExtension [_command, _arguments];
private _payload = _result param [0, "", [""]];
private _code = _result param [1, -1, [0]];
private _success = _code isEqualTo 0 && { (_payload find "Error:") != 0 };

if !(_success) then {
    ERROR_3("Extension command %1 failed with code %2: %3",_command,_code,_payload);
};

[_payload, _success]
