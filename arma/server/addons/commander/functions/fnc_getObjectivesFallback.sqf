#include "..\script_component.hpp"

/*
 * File: fnc_getObjectivesFallback.sqf
 * Description: Scans for common mission markers to use as strategic objectives.
 *              Checks well-known names first, then scans allMapMarkers for "obj_" prefix,
 *              and falls back to map quadrant positions as a last resort.
 */

private _positions = [];

// Check well-known marker names
private _markerCandidates = ["obj_center", "obj1", "obj2", "objective", "AO", "capture_pos", "defend_pos"];
{
    if ((markerShape _x) isNotEqualTo "") then {
        _positions pushBack (getMarkerPos _x);
    };
} forEach _markerCandidates;

// Scan all markers for "obj_" prefix if no known markers found
if (_positions isEqualTo []) then {
    {
        if ((_x select [0, 4]) isEqualTo "obj_" && { (markerShape _x) isNotEqualTo "" }) then {
            _positions pushBack (getMarkerPos _x);
        };
    } forEach allMapMarkers;
};

// Last resort: use map quadrant positions (skip water)
if (_positions isEqualTo []) then {
    private _worldSize = worldSize;
    private _candidates = [
        [_worldSize * 0.25, _worldSize * 0.25, 0],
        [_worldSize * 0.75, _worldSize * 0.25, 0],
        [_worldSize * 0.5, _worldSize * 0.75, 0],
        [_worldSize * 0.5, _worldSize * 0.5, 0]
    ];

    {
        if !(surfaceIsWater _x) then {
            _positions pushBack _x;
        };
    } forEach _candidates;

    // If all candidates are water, use map center as absolute fallback
    if (_positions isEqualTo []) then {
        _positions = [[_worldSize * 0.5, _worldSize * 0.5, 0]];
    };
};

_positions
