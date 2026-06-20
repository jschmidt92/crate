#include "..\script_component.hpp"

/*
 * File: fnc_transport.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Stages oversized extension requests and assembles chunked extension responses.
 *
 * Arguments:
 * 0: [STRING] - Transport mode
 *
 * Return Value:
 * Transport result [ARRAY]
 *
 * Example:
 * ["assemble", _response, _success, _prefix, _prefixLength, _invoker] call forge_crate_extension_fnc_transport;
 */

params [["_mode", "", [""]]];

switch (_mode) do {
    case "stage": {
        _this params [
            "_mode",
            ["_transportCommand", "", [""]],
            ["_argumentsJson", "", [""]],
            ["_chunkSize", 12000, [0]],
            ["_invoker", {}, [{}]]
        ];

        private _transferId = format [
            "req_%1_%2",
            floor (diag_tickTime * 1000),
            floor (random 1000000000)
        ];

        for "_offset" from 0 to ((count _argumentsJson) - 1) step _chunkSize do {
            private _chunk = _argumentsJson select [_offset, _chunkSize];
            ["transport:request:append", [_transferId, _chunk]] call _invoker params ["_result", "_success"];
            if (!_success || {(_result find "Error:") == 0}) exitWith {
                _transferId = "";
            };
        };

        if (_transferId isEqualTo "") exitWith { ["", [], false] };

        ["transport:invoke_stored", [_transportCommand, _transferId], true]
    };

    case "assemble": {
        _this params [
            "_mode",
            ["_response", "", [""]],
            ["_responseSuccess", false, [true]],
            ["_chunkPrefix", "", [""]],
            ["_chunkPrefixLength", 0, [0]],
            ["_invoker", {}, [{}]]
        ];

        if !(_responseSuccess && {(_response find _chunkPrefix) == 0}) exitWith {
            [_response, _responseSuccess]
        };

        private _envelope = fromJSON (_response select [_chunkPrefixLength]);
        if !(_envelope isEqualType createHashMap) exitWith {
            ["Error: Invalid extension chunk envelope", false]
        };

        private _transferId = _envelope getOrDefault ["transferId", ""];
        private _chunkCount = _envelope getOrDefault ["chunkCount", 0];
        if (_transferId isEqualTo "" || {!(_chunkCount isEqualType 0)} || {_chunkCount < 1}) exitWith {
            ["Error: Invalid extension chunk metadata", false]
        };

        private _assembled = "";
        private _readSuccess = true;

        for "_index" from 0 to (_chunkCount - 1) do {
            ["transport:response:get", [_transferId, str _index]] call _invoker params ["_chunk", "_success"];
            if (!_success || {(_chunk find "Error:") == 0}) exitWith {
                _readSuccess = false;
                _assembled = "Error: Failed to retrieve chunked extension response";
            };
            _assembled = _assembled + _chunk;
        };

        ["transport:response:clear", [_transferId]] call _invoker;
        [_assembled, _readSuccess]
    };

    default {
        ["Error: Unsupported extension transport mode", false]
    };
};
