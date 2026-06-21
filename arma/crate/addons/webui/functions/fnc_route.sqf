#include "..\script_component.hpp"

/*
 * File: fnc_route.sqf
 * Author: IDSolutions
 * Date: 2026-06-18
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Parses a browser JSDialog message and routes supported WebUI events to client or server handlers.
 *
 * Arguments:
 * 0: [CONTROL] - Web browser control
 * 1: [BOOL] - Whether the message originated from a confirm dialog
 * 2: [STRING] - Browser message payload
 *
 * Return Value:
 * Event handled [BOOL]
 *
 * Example:
 * [_control, false, _message] call forge_crate_webui_fnc_route;
 */

params [
    ["_control", controlNull, [controlNull]],
    ["_isConfirmDialog", false, [false]],
    ["_message", "", [""]]
];

diag_log format ['FORGE_UI_EVENTS: control: %1, isConfirmDialog: %2, message: %3', _control, _isConfirmDialog, _message];

if (_message isEqualTo "") exitWith { false };

private _payload = createHashMap;
private _parsed = false;

if ((_message select [0, 1]) in ["{", "["]) then {
    _payload = fromJSON _message;
    _parsed = true;
};

private _event = if (_parsed && { _payload isEqualType createHashMap }) then {
    _payload getOrDefault ["event", "ui::message"]
} else {
    "ui::message"
};

if (_event isEqualTo "ui::close") exitWith {
    private _display = ctrlParent _control;
    if !(isNull _display) then { _display closeDisplay 2; };

    true
};

private _parts = _event splitString ":";
private _namespace = if (_parts isNotEqualTo []) then { _parts select 0 } else { "" };

switch (_namespace) do {
    case "bank": {
        private _requestId = _payload getOrDefault ["requestId", ""];
        private _data = _payload getOrDefault ["data", createHashMap];

        if (_requestId isNotEqualTo "" && { !isNull player }) then {
            [SRPC(webui,bankRequest), [player, _requestId, _event, _data]] call CFUNC(serverEvent);
        };
    };
    default {
        // Empty for the moment
    };
};

true
