#include "script_component.hpp"

/*
 * Bridge extension callbacks into Forge-owned CBA server events.
 *
 * Rust callbacks should use a Forge namespace:
 *   forge:<feature>, <callback>, <json payload>
 */
addMissionEventHandler ["ExtensionCallback", {
    params [["_name", "", [""]], ["_function", "", [""]], ["_data", "", [""]]];

    private _topic = "";
    if ((_name find "forge:") == 0) then { _topic = _name select [6]; };
    if (_topic isEqualTo "" || { _function isEqualTo "" }) exitWith {};

    private _payload = _data;
    if (_data isEqualType "" && {count _data > 0}) then {
        private _first = _data select [0, 1];
        if (_first in ["{", "["]) then { _payload = fromJSON _data; };
    };

    private _eventTopic = (_topic splitString ":") joinString "_";
    private _eventName = format ["forge_crate_%1_%2", _eventTopic, _function];

    [_eventName, [_payload, _name, _function]] call CFUNC(localEvent);
}];
