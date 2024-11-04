# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.7.0...HEAD
[6.7.0]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.1...v6.7.0
[6.6.1]: https://github.com/7H3LaughingMan/wayfinder/compare/v6.6.0...v6.6.1
[6.6.0]: https://github.com/7H3LaughingMan/wayfinder/releases/tag/v6.6.0
