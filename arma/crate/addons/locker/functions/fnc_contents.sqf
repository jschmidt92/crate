#include "..\script_component.hpp"

/*
 * File: fnc_contents.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Normalizes a cargo container into classname-keyed locker commodities.
 *
 * Arguments:
 * 0: Cargo container [OBJECT]
 *
 * Return Value:
 * Locker commodity map [HASHMAP]
 *
 * Example:
 * [_container] call forge_crate_locker_fnc_contents;
 */

params [["_root", objNull, [objNull]]];
if (isNull _root) exitWith { createHashMap };

private _items = createHashMap;
private _add = {
    params [
        ["_category", "items", [""]],
        ["_classname", "", [""]],
        ["_amount", 1, [0]],
        ["_ammo", -1, [0]]
    ];
    if (_classname isEqualTo "" || {_amount <= 0}) exitWith {};

    private _item = _items getOrDefault [_classname, createHashMapFromArray [
        ["category", _category],
        ["classname", _classname],
        ["amount", 0],
        ["ammo", 0]
    ]];
    _item set ["amount", (_item get "amount") + _amount];
    if (_ammo >= 0) then {
        _item set ["ammo", (_item get "ammo") + _ammo];
    };
    _items set [_classname, _item];
};

private _captureCargo = {
    params [["_container", objNull, [objNull]]];
    if (isNull _container) exitWith {};

    {
        _x params [
            ["_weapon", "", [""]],
            ["_muzzle", "", [""]],
            ["_pointer", "", [""]],
            ["_optic", "", [""]],
            ["_primaryMagazine", [], [[]]],
            ["_secondaryMagazine", [], [[]]],
            ["_bipod", "", [""]]
        ];

        ["weapons", _weapon] call _add;
        { ["items", _x] call _add } forEach [_muzzle, _pointer, _optic, _bipod];
        {
            if (_x isNotEqualTo []) then {
                ["magazines", _x param [0, ""], 1, _x param [1, 0]] call _add;
            };
        } forEach [_primaryMagazine, _secondaryMagazine];
    } forEach weaponsItemsCargo _container;

    {
        ["magazines", _x param [0, ""], 1, _x param [1, 0]] call _add;
    } forEach magazinesAmmoCargo _container;

    private _nestedContainers = everyContainer _container;
    private _containerCounts = createHashMap;
    {
        private _containerClass = _x select 0;
        _containerCounts set [
            _containerClass,
            (_containerCounts getOrDefault [_containerClass, 0]) + 1
        ];
    } forEach _nestedContainers;

    getItemCargo _container params ["_itemClasses", "_itemAmounts"];
    {
        private _amount = (_itemAmounts select _forEachIndex) - (_containerCounts getOrDefault [_x, 0]);
        ["items", _x, _amount max 0] call _add;
    } forEach _itemClasses;

    {
        _x params ["_containerClass", "_nestedContainer"];
        private _category = ["items", "backpacks"] select (isClass (configFile >> "CfgVehicles" >> _containerClass));
        [_category, _containerClass] call _add;
        [_nestedContainer] call _captureCargo;
    } forEach _nestedContainers;
};

[_root] call _captureCargo;
_items
