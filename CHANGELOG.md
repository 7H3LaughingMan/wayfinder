# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

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
- When adding a waypoint the entrie found path will be added as multiple waypoints

### Fixed

- Make sure found path is properly snapped to the grid
- Fixed a problem with checking fog exploration where it was slightly off when checking pixels
- Improved Fog Exploration

[Unreleased]: https://github.com/7H3LaughingMan/wayfinder/compare/v7.0.0...HEAD
[7.0.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.8.1...v7.0.0
[6.8.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.8.0...v6.8.1
[6.8.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.2...v6.8.0
[6.7.2]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.1...v6.7.2
[6.7.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.0...v6.7.1
[6.7.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.1...v6.7.0
[6.6.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.0...v6.6.1
[6.6.0]: https://github.com/7H3LaughingMan/wayfinder/releases/tag/v6.6.0
