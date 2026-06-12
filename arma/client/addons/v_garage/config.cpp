#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        author = AUTHOR;
        authors[] = {"J.Schmidt"};
        url = ECSTRING(main,url);
        name = COMPONENT_NAME;
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "forge_client_main",
            "forge_client_common"
        };
        units[] = {};
        weapons[] = {};
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
