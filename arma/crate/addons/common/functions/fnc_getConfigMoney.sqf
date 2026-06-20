#include "..\script_component.hpp"

/*
 * Reads a money value from mission config as a string.
 */

params [["_config", configNull, [configNull]], ["_default", "0.00", [""]]];

if (isText _config) exitWith { getText _config };
if (isNumber _config) exitWith { str getNumber _config };

_default
