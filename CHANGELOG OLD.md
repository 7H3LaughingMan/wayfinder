# 6.5.3

- Fix for hexagonal token shapes not being centered.

# 6.5.2

- Fix "Export Fog Exploration" macro
- Added Polish Translation (Thanks Lioheart)

# 6.5.1

- The path found by other users is now visible to everyone when the ruler is visible to them.

# 6.5.0

- Dropping the PF2e portion of the module name and renaming it to Wayfinder. This module is going to support SF2e as well when it becomes it's own system.

# 6.4.1

- Added a macro that can be used to export an image of the Fog Exploration.

# 6.4.0

- When trying to find a path on a SquareGrid it will attempt to try and follow a direct path if possible.

# 6.3.0

- Now checks the history and waypoints to determine where on the 5-10-5 diagonal rule it's currently at before it tries to find a path.

# 6.2.3

- Removed some debug logging that made it in by mistake.

# 6.2.2

- Fixes a small problem where it wasn't calculating distances properly.

# 6.2.1

- Fixes a small problem with the localization.

# 6.2.0

- Removed support for Token Vision on a scene, was causing issues with performance and pathfinding.
- When retrieving the Fog Exploration overlay it now scales it down to 5%, this is to prevent performance issues when you start dragging a token. There might still be a slight stutter on stupidly big maps.
- Added a setting for enforcing Fog Exploration when pathfinding.
- If GM Vision is enabled or if a token doesn't have vision enabled it will ignore fog exploration. Make sure your player's tokens have vision enabled!
- Lastly, fixed an issue where tiny creatures weren't defaulting to the same token space as small/medium creatures.

# 6.1.0

- Adds support for Token Vision & Fog Exploration.
  - If Token Vision is enabled for a scene it will not find a path going through squares that are not visible.
  - If Fog Exploration is enabled for a scene it will not find a path going through squares that haven't been explored.
  - If both are enabled then it will not find a path going through squares that are not visible and haven't been explored.

# 6.0.0

Initial Release! This is the successor to PF2e Token Drag Ruler and adds pathfinding capabilities to the token drag measurement tool added to the PF2e System.

- Uses a custom Rust WebAssembly that runs the A* Pathfinding Algorithm, provides faster results in milliseconds compared to trying to run the same algorithm via JavaScript.
- If a path can't be found it will default back to acting like it's not enabled.
- Only supports square grids properly at the moment, the built in token drag measurement tool does not support hexagonal grids at the moment.
- Collision detection works by getting the shape of the token and reducing it by 40% of the grid's size.
  - For example on a 100 grid size map a medium creature would be 100 x 100, this is then reduced by 40 pixels on each side down to a 20 x 20 square. While a large creature starts with a 200 x 200 square and is reduced down to a 120 x 120 square.
- This module does not account for "squeezing", you will have to manually adjust the token's size using effects in order to have a large creature navigate tighter paths.