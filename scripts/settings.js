import { MODULE_ID } from "./const.js";

export function registerSettings() {
    game.settings.register(MODULE_ID, "enablePathfinding", {
        scope: "client",
        config: false,
        type: Boolean,
        default: false
    });

    game.settings.register(MODULE_ID, "fogExploration", {
        name: "pf2e-compass.settings.fogExploration.name",
        hint: "pf2e-compass.settings.fogExploration.hint",
        scope: "world",
        config: true,
        type: Boolean,
        default: true
    })
}