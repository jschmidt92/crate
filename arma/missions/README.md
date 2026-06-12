# Forge Mission Config

Copy `forge_mission_config.hpp` into the mission folder and include it from `description.ext`:

```cpp
#include "forge_mission_config.hpp"
```

`CfgForgeMission >> Actor` controls first-login actor defaults. Money values should stay quoted strings so Arma does not round large or precise values before Rust receives them.

Virtual arsenal unlocks use the mission config label `Locker`, matching the previous framework shape. Use category arrays under `CfgForgeMission >> Actor >> Locker`, for example `weapons[] = {"hgun_P07_F"};`.

Virtual garage unlocks are category arrays under `CfgForgeMission >> Actor >> VirtualGarage`, for example `cars[] = {"B_Quadbike_01_F"};`.
