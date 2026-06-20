#include "..\script_component.hpp"

/*
 * File: fnc_send.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Serializes a Forge response and dispatches it to the active WebUI browser host callback.
 *
 * Arguments:
 * 0: [HASHMAP] - Response payload
 *
 * Return Value:
 * Response dispatched [BOOL]
 *
 * Example:
 * [_response] call forge_crate_webui_fnc_send;
 */

params [["_response", createHashMap, [createHashMap]]];

diag_log format ['FORGE_SEND_TO_BROWSER: response: %1', toJSON _response];

disableSerialization;
private _display = uiNamespace getVariable [QGVAR(display), displayNull];
if (isNull _display) exitWith { false };

private _browser = _display displayCtrl 78001;
if (isNull _browser) exitWith { false };

private _script = format ["window.forgeHostReceive(%1);", toJSON _response];
_browser ctrlWebBrowserAction ["ExecJS", _script];
true
