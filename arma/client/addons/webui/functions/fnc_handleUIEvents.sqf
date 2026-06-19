#include "..\script_component.hpp"

params [
    ["_control", controlNull, [controlNull]],
    ["_isConfirmDialog", false, [false]],
    ["_message", "", [""]]
];

if (_message isEqualTo "") exitWith { false };

private _payload = createHashMap;
private _parsed = false;

if ((_message select [0, 1]) in ["{", "["]) then {
    _payload = fromJSON _message;
    _parsed = true;
};

private _event = if (_parsed && {_payload isEqualType createHashMap}) then {
    _payload getOrDefault ["event", "ui::message"]
} else {
    "ui::message"
};

if (_event isEqualTo "ui::close") exitWith {
    closeDialog 0;
    true
};

true
