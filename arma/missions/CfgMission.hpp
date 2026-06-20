/*
 * Include this file from description.ext:
 * #include "CfgMission.hpp"
 */

class CfgMission {
    Commander[] = {
        {"reassignCadence", 20},
        {"spawnConsiderCadence", 45},
        {"threatRecalcCadence", 10},
        {"maxInfantryGroups", 6},
        {"maxArmorGroups", 2},
        {"maxSupportGroups", 2},
        {"objectiveRecalcDistance", 2000},
        {"objectiveMarkerFallbackRadius", 1500},
        {"spawnMinDistance", 800},
        {"spawnMaxDistance", 2200},
        {"minReassignInterval", 60},
        {"infantryGroupSize", 8},
        {"armorCrewGroupSize", 4},
        {"supportGroupSize", 6},
        {"highThreatThreshold", 0.65},
        {"medThreatThreshold", 0.35}
    };

    class Services {
        class Refuel {
            regularPricePerLiter = "1.00";
            jeta1PricePerLiter = "1.80";
        };

        class Repair {
            fullRepairPrice = "2500.00";
        };

        class Rearm {
            unitPrice = "75.00";
        };

        class Medical {
            respawnPrice = "0.00";
            fullHealPrice = "250.00";
        };
    };

    class DefaultOrganization {
        startingBank = "0.00";
        paydayAmount = "0.00";

        class Locker {
            items[] = {};
            weapons[] = {};
            magazines[] = {};
            backpacks[] = {};
        };

        class VirtualGarage {
            cars[] = {};
            armor[] = {};
            helis[] = {};
            planes[] = {};
            naval[] = {};
            other[] = {};
        };
    };

    class Actor {
        startingCash = "0.00";
        startingBank = "0.00";
        loadout[] = {
            {},
            {},
            {"hgun_P07_F", "", "", "", {"16Rnd_9x21_Mag", 17}, {}, ""},
            {"U_BG_Guerrilla_6_1", {{"FirstAidKit", 2}, {"ACE_EarPlugs", 1}}},
            {"V_Rangemaster_belt", {{"16Rnd_9x21_Mag", 4, 17}}},
            {},
            "H_Cap_blk_ION",
            "",
            {"Binocular", "", "", "", {}, {}, ""},
            {"ItemMap", "ItemGPS", "ItemRadio", "ItemCompass", "ItemWatch", ""}
        };

        class Locker {
            items[] = {
                "FirstAidKit",
                "G_Combat",
                "H_Cap_blk_ION",
                "H_HelmetB",
                "ItemCompass",
                "ItemGPS",
                "ItemMap",
                "ItemRadio",
                "ItemWatch",
                "NVGoggles",
                "U_BG_Guerrilla_6_1",
                "V_Rangemaster_belt",
                "V_TacVest_oli",
                "ACE_EarPlugs"
            };
            weapons[] = {
                "arifle_MX_F",
                "hgun_P07_F",
                "Binocular"
            };
            magazines[] = {
                "16Rnd_9x21_Mag",
                "30Rnd_65x39_caseless_black_mag",
                "Chemlight_blue",
                "Chemlight_green",
                "Chemlight_red",
                "Chemlight_yellow",
                "HandGrenade",
                "SmokeShell",
                "SmokeShellBlue",
                "SmokeShellGreen",
                "SmokeShellOrange",
                "SmokeShellPurple",
                "SmokeShellRed",
                "SmokeShellYellow"
            };
            backpacks[] = {
                "B_AssaultPack_rgr"
            };
        };

        class VirtualGarage {
            cars[] = {
                "B_Quadbike_01_F"
            };
            armor[] = {};
            helis[] = {};
            planes[] = {};
            naval[] = {};
            other[] = {};
        };
    };
};
