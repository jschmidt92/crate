#include "..\script_component.hpp"

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
