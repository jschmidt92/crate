#include "..\script_component.hpp"

/*
 * File: fnc_configMoney.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Reads a text or numeric money value from mission configuration and returns its string form.
 *
 * Arguments:
 * 0: [CONFIG] - Money configuration entry
 * 1: [STRING] - Fallback amount (default: "0.00")
 *
 * Return Value:
 * Money amount [STRING]
 *
 * Example:
 * [_config >> "startingBank", "0.00"] call forge_crate_common_fnc_configMoney;
 */

params [["_config", configNull, [configNull]], ["_default", "0.00", [""]]];

if (isText _config) exitWith { getText _config };
if (isNumber _config) exitWith { str getNumber _config };

_default
