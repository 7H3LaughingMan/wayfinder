# Wayfinder

[![](https://img.shields.io/badge/License-MIT-D3D3D3)](LICENSE.md)
[![](https://gitlocalize.com/repo/9859/whole_project/badge.svg)](https://gitlocalize.com/repo/9859?utm_source=badge)
[![](https://img.shields.io/badge/Support%20me%20on%20Ko--fi-FF5E5B?logo=ko-fi&logoColor=FFFFFF)](https://ko-fi.com/7h3laughingman)

![GitHub Downloads (specific asset, all releases)](https://img.shields.io/github/downloads/7H3LaughingMan/wayfinder/module.zip)
![GitHub Downloads (specific asset, latest release)](https://img.shields.io/github/downloads/7H3LaughingMan/wayfinder/latest/module.zip)
![Forge Installs](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fforge-vtt.com%2Fapi%2Fbazaar%2Fpackage%2Fwayfinder&query=package.installs&suffix=%25&label=Forge%20Installs&color=4aa94a)

A wayfinder for your everyday needs! It let's you navigate treacherous paths and plan the shortest route to your destination.

![Wayfinder Example](https://github.com/7H3LaughingMan/wayfinder/raw/refs/heads/main/toolclips/pathfinding.webp)

## Basic Troubleshooting

- You need to make sure that you have "Pathfinding" is enabled. Under the "Token Controls" on the left hand side you should see an option that looks like a compass, make sure this is enabled (it should look like it's lighten up).

- Wayfinder is designed to only worked on maps that have a grid, which means maps setup with either a Hexagonal or Square grid. Gridless maps are bit more complicated and require a lot more work to implement pathfinding that works cleanly.

- If have "Unconstrained Movement" (Ghost Icon) enabled then Wayfinder will not attempt to try and find a path movement through walls has been enabled.

- In order for Wayfinder to force a path through explored parts of a map you need to make sure the following are enabled. If any of these aren't enabled than Wayfinder will try to find a path through unexplored parts of the map.
  1. "Fog Exploration Restriction" is enabled in Wayfinder's settings.
  2. The scene that is currently active has both "Token Vision" and "Fog Exploration" checked, you can verify this by opening the scene's configuration menu and looking under the "Lighting" tab.
  3. The active token that Wayfinder is trying to find a path for must have "Vision Enabled" checked. You can verify this by opening the token's configuration menu and looking under the "Vision" tab.
