#include "..\script_component.hpp"

/*
 * File: fnc_open.sqf
 * Author: IDSolutions
 * Date: 2026-06-18
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Opens the Forge WebUI display, registers its browser event bridge, and loads the packaged site.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Forge WebUI display, or displayNull when it cannot be opened [DISPLAY]
 *
 * Example:
 * call forge_crate_webui_fnc_open;
 */

disableSerialization;

private _display = uiNamespace getVariable [QGVAR(display), displayNull];
if !(isNull _display) exitWith { _display };

private _parent = findDisplay 46;
if (isNull _parent) exitWith { displayNull };

_display = _parent createDisplay "Forge_WebUI_Display";
if (isNull _display) exitWith { displayNull };

private _browser = _display displayCtrl 78001;
if (isNull _browser) exitWith { _display };

_browser ctrlAddEventHandler ["JSDialog", {
    params ["_control", "_isConfirmDialog", "_message"];
    [_control, _isConfirmDialog, _message] call FUNC(route);
}];

private _path = QPATHTOF2(ui\_site\index.html);
_browser ctrlWebBrowserAction ["LoadFile", _path];

_display
