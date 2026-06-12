#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        author = AUTHOR;
        authors[] = {"J.Schmidt"};
        url = ECSTRING(main,url);
        name = COMPONENT_NAME;
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "forge_server_main",
            "forge_server_common",
            "forge_server_extension",
            "forge_server_garage"
        };
        units[] = {};
        weapons[] = {};
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
