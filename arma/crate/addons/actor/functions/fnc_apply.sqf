#include "..\script_component.hpp"

/*
 * File: fnc_apply.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Applies the authoritative actor state returned by the Rust extension. New actors are stripped
 * before receiving the configured default loadout; returning actors receive their persisted state.
 *
 * Arguments:
 * 0: [OBJECT] - Local player object
 * 1: [HASHMAP] - Persisted actor state
 * 2: [BOOL] - Whether this actor was newly created
 *
 * Return Value:
 * State applied [BOOL]
 *
 * Example:
 * [player, _actor, false] call forge_crate_actor_fnc_apply;
 */

params [
    ["_player", objNull, [objNull]],
    ["_actor", createHashMap, [createHashMap]],
    ["_created", false, [false]]
];

if (isNull _player || { !local _player } || { _actor isEqualTo createHashMap }) exitWith { false };

private _useDefaultLoadout = _created || { !GVAR(persistLoadout) };
if (_useDefaultLoadout) then {
    removeAllWeapons _player;
    removeAllItems _player;
    removeAllAssignedItems _player;
    removeUniform _player;
    removeVest _player;
    removeBackpack _player;
    removeHeadgear _player;
    removeGoggles _player;
};

private _loadout = if (_useDefaultLoadout) then {
    getArray (missionConfigFile >> "CfgMission" >> "Actor" >> "loadout")
} else {
    _actor getOrDefault ["loadout", []]
};
if (_loadout isEqualType [] && { _loadout isNotEqualTo [] }) then {
    _player setUnitLoadout [_loadout, false];
};

if (!_created && { GVAR(persistPosition) }) then {
    private _position = _actor getOrDefault ["position", []];
    if (_position isEqualType [] && { count _position isEqualTo 3 }) then {
        _player setPosASL _position;

        private _positionATL = getPosATLVisual _player;
        private _altitudeATL = _positionATL select 2;
        private _verticalVelocity = (velocity _player) select 2;
        if (_altitudeATL > 5 && { _verticalVelocity < 0 }) then {
            _player setVelocity [0, 0, 0];
            _player setPosATL [_positionATL select 0, _positionATL select 1, 1];
        };
    };

};

if !(_created) then {
    private _direction = _actor getOrDefault ["direction", getDir _player];
    if (_direction isEqualType 0) then { _player setDir _direction; };

    private _rank = _actor getOrDefault ["rank", "PRIVATE"];
    if (_rank in ["PRIVATE", "CORPORAL", "SERGEANT", "LIEUTENANT", "CAPTAIN", "MAJOR", "COLONEL"]) then {
        _player setUnitRank _rank;
    };

    private _stance = _actor getOrDefault ["stance", "STAND"];
    private _stanceAction = switch (_stance) do {
        case "CROUCH": { "Crouch" };
        case "PRONE": { "Prone" };
        default { "Stand" };
    };
    _player playActionNow _stanceAction;
};

if (_actor getOrDefault ["holster", true]) then {
    _player action ["SwitchWeapon", _player, _player, 100];
};

true
