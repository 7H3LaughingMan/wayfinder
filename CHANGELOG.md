# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [14.1.3] - 2026-09-30

### Changed

- Removed maze generation, this was mostly used for debug purposes
- Fog exploration data is now stored as a BitVec (Bit Vector)
  - Previously Wayfinder was storing a vector of bytes that represented the red channel similiar to how Foundry does it, that means a 3,200 x 3,200 map result in a vector containg 10,240,000 bytes. Then it was using that data to check the center pixel and it's surronding pixels of a grid to see if a point was "explored".
  - It now pre-processes the fog exploration texture and generates a vector where each element represents a single grid space and if it's been "explored". The difference between a bit vector and a vector of booleans is just how the underlaying data is stored, while a boolean can either be true or false it still takes up an entire byte but a bit vector packs the data so that a single byte can represent eight booleans. So for a 32 x 32 grid would result in a boolean vector that contains 1,024 bytes, but a bit vector would only be 128 bytes.

### Fixed

- When generating the fog exploration texture that Wayfinder uses we not account for textures that result in a single row of pixel data that ends up being padded with extra bytes. By default WebGL sets PACK_ALIGNMENT to 4, meaning is a single row of pixel data isn't divisible by 4 bytes it will pad the row with extra data to make it divisible.

## [14.1.2] - 2026-09-27

### Changed

- Optimized some sorting by simplifying some code which results in increased performance, thanks [fotoply](https://github.com/fotoply)!

## [14.1.1] - 2026-09-26

### Fixed

- When rendering the fog sprite texture make sure we "translate" it so that we remove padding
- Make sure we can actually find a "direct path" when calculating a path between two points when diagonal movement is illegal

## [14.1.0] - 2026-09-25

### Changed

- Reworked how fog exploration is handled to improve performance
  - Previously this worked by creating a texture of the fog exploration texture that is 25% the size of the map, extracting the pixels, but since it returned RGBA data it then had to be further processed to only extract the red pixel data since that is what is used by Foundry. On a large map such as 36,000 x 48,000 it was generating a 9,000 x 12,000 texture, extracting the pixels, and processing them which was taking around 800 ms.
  - The texture that Wayfinder created is now the same size as Foundry's fog exploration texture which is scaled down so the maximum width/height is 4,096 pixels, so a 36,000 x 48,000 map used a fog exploration texture of 3,072 x 4,096. (Which means previously Wayfinder was upscaling the 3,072 x 4,096 texture to 9,000 x 12,000 which was a waste.) This texture also only has the red channel so when the pixels are extracted there is only red pixel data which means it doesn't need to be processed further. With these improvements updating the fog exploration data stored by Wayfinder now only takes 6.354 ms for the 36,000 x 48,000 map. The only downside is when it comes to smaller maps such as a 2,000 x 1,500 map it went from 1.75 ms to 3.878 ms since it's now processing a texture that is 2,000 x 1,500 instead of 500 x 375, but I figure keeping the texture that is used by Wayfinder as the same size as Foundry's fog exploration texture is better.

## [14.0.1] - 2026-05-12

### Fixed

- If a wall is not assigned to any level it will now count as being on all levels which is intended according to Foundry.

## [14.0.0] - 2026-04-13

This is an update to add support for Foundry Version 14, there is no backwards compatibility. One thing to note is that Foundry doesn't provide a means to drag a token from one level to another, it has to be done either through regions or manually changing which level a token is on. It is possible to control tokens on another level and Wayfinder will make sure that those tokens avoid walls on their level. However, Fog Exploration is only available for the currently active level so if you are restricting movement based on it then tokens on another level will not be able to move into areas that are explored on their level that aren't explored on the active level.

## [13.1.1] - 2026-04-10

### Changed

- Adjusted the order in which neighboring nodes are returned, they are sorted using the rectilinear distance to the goal so nodes closest to the goal should be processed first

## [13.1.0] - 2026-04-09

### Added

- 4th Dimensional Navigation
  - When attempting to find a path it will now generate and walk from the current node to the goal, if this path is valid and doesn't run into any walls or enter unexplored parts of the map it will treat the goal as being "next" to the current node with the actual cost to move down the path as the cost of moving to the goal. That means if it thinks there is a cheaper route it will try and follow that first, but if it ends up costing the same as the direct path it will use that.

### Changed

- When the Grid Diagonal Rule is set to Exact it will instead treat it as Approximate, this doesn't change anything on Foundry's side but just changes how Wayfinder calculates the cost which allows me to do the next bit.
- Costs are now in a "decimal" format, so instead of using a normal floating point numeric that can lose precision it nows uses a fixed decimal that doesn't lose precision as easly. Meaning `0.1 + 0.2 = 0.3` instead of `0.1 + 0.2 = 0.30000000000000004`. With the new changes above we only need 2-4 decimals of precision which doesn't get lost.

## [13.0.3] - 2026-04-02

### Changed

- When getting neighboring nodes, return nodes that don't require diagonal movement first before returning ones that do require diagonal movement

## [13.0.2] - 2026-04-01

### Fixed

- FogManager now accounts for the "scale" of the fog sprite when attempting to extract the exploration data

## [13.0.1] - 2026-03-31

### Fixed

- Fixed a typo that prevented Wayfinder from getting the fog of war properly

## [13.0.0] - 2026-03-31

This is pretty much a complete re-write of the Rust side of things in order to make the module not dependent on a specific system.

## [7.1.0] - 2025-06-20

### Changed

- Reworked Fog Exploration
  - Due to a [bug](https://github.com/foundryvtt/foundryvtt/issues/13046) with `FogManager` the way Wayfinder handles fog exploration needed to be reworked.
  - It will now generate a texture that represents the explored area and use that to calculate if a spot has been "explored". This is similar to how it used to work but instead of extracting all four channels (red, green, blue, and alpha) it only extracts the red channel. It is also done less often now, it used to be done every time Wayfinder was trying to find a path and now it's only done when the canvas is ready or when the explored area has changed.

### Fixed

- Wayfinder now respects if you have pathfinding disabled

## [7.0.0] - 2025-06-17

Foundry Virtual Tabletop - Version 13 Support!

### Added

- Support for the new Token Ruler

### Changed

- Project Refactorization

### Removed

- Difficult Terrain Support (This will need to be readded by the PF2e System)
- Action Icons (This will need to be implemented by the PF2e System)

## [6.8.1] - 2024-12-13

### Fixed

- Fixed a problem where Sequencer appears to be changing PixiJS GLTexture Index

## [6.8.0] - 2024-12-13

### Added

- Added they Wayfinder object (`canvas.wayfinder`) to the canvas
  - This is created when the canvas is ready and stores information about the current scene such as the bounds, grid, and walls. When the canvas is torn down the memory is freed and the object is cleared.
- Wall creation, deletion, or updates are passed to the Wayfinder object
  - This is used to keep the stored data about the walls for collision detection up to date.
- Added [QuadTree](https://en.wikipedia.org/wiki/Quadtree)
  - This is a bit complicated to explain but it's an efficient way to store objects in 2D space and allows you to retrieve objects in a certain region without checking every object, this is used to store information about the walls for collision detection.
- If someone is using Wayfinder to find a path while moving a token, the full path is now transmitted

### Changed

- Changed data from f32 to f65 to match JavaScript's Number
- The explored texture is passed to the Wayfinder object to read the pixel data in WebAssembly space
  - This is to prevent reading the data in JavaScript space and then passing the raw data to WebAssembly space which can be slow.
- Collision detection is handled in the same fashion as Foundry now
  - It's a single test from the center of the token to the center of where the token will be when moved. This does mean that tokens that take up more than one grid can probably pass through smaller pathways but these are "valid" moves according to Foundry for the time being.
- If you are using the regular ruler to measure distances, Wayfinder will not interfere in any way now

### Removed

- Removed Physics Engine - Reduces WebAssembly from 256 KB to 99.9 KB

## [6.7.2] - 2024-11-05

### Added

- Added known module conflicts.

### Changed

- Action icons will only display if the grid's scale is set to 5 ft.

## [6.7.1] - 2024-11-05

### Added

- Added a custom font that is used for the action icons.

### Fixed

- A problem with it locking up when trying to move a create with 0 land speed.

## [6.7.0] - 2024-11-04

### Added

- Action Icons, when enabled an icon will be display when moving a token that represents how many strides it will cost based on the token's land speed.
- Difficult Terrain, when enabled it will show the cost of moving through difficult terrain. (Does not impact pathfinding at the moment)
- Movement History, when enabled it will keep track of a token's movement during combat and reset at the start of the token's turn. (Only works with tokens in the encounter tracker at the moment)
  - The GM can reset a token's movement history by right-clicking the combatant in the encounter tracker and selecting the `Clear Movement History` option. If you are using PF2e HUD's encounter tracker press `Ctrl` to find this option.

## [6.6.1] - 2024-10-21

### Fixed

- Oops, mixed up X/Y coordinates as X/X coordinates

## [6.6.0] - 2024-10-21

### Changed

- Pathfinding toggle is now a compass
- When adding a waypoint the entire found path will be added as multiple waypoints

### Fixed

- Make sure found path is properly snapped to the grid
- Fixed a problem with checking fog exploration where it was slightly off when checking pixels
- Improved Fog Exploration

[14.1.3]: https://github.com/7H3LaughingMan/wayfinder/compare/v14.1.2...v14.1.3
[14.1.2]: https://github.com/7H3LaughingMan/wayfinder/compare/v14.1.1...v14.1.2
[14.1.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v14.1.0...v14.1.1
[14.1.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v14.0.1...v14.1.0
[14.0.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v14.0.0...v14.0.1
[14.0.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.1.1...v14.0.0
[13.1.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.1.0...v13.1.1
[13.1.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.0.3...v13.1.0
[13.0.3]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.0.2...v13.0.3
[13.0.2]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.0.1...v13.0.2
[13.0.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v13.0.0...v13.0.1
[13.0.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v7.1.0...v13.0.0
[7.1.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v7.0.0...v7.1.0
[7.0.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.8.1...v7.0.0
[6.8.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.8.0...v6.8.1
[6.8.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.2...v6.8.0
[6.7.2]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.1...v6.7.2
[6.7.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.0...v6.7.1
[6.7.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.1...v6.7.0
[6.6.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.0...v6.6.1
[6.6.0]: https://github.com/7H3LaughingMan/wayfinder/releases/tag/v6.6.0
