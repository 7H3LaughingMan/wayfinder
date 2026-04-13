import { CancellationToken, Wayfinder } from "../pkg/wayfinder_wasm";

Hooks.once("init", () => {
    game.settings.register("wayfinder", "enablePathfinding", {
        name: "enablePathfinding",
        scope: "user",
        config: false,
        type: Boolean,
        default: false
    });

    game.settings.register("wayfinder", "fogExploration", {
        name: "wayfinder.settings.fogExploration.name",
        hint: "wayfinder.settings.fogExploration.hint",
        scope: "world",
        config: true,
        type: Boolean,
        default: true
    });
});

Hooks.once("ready", () => {
    libWrapper.register("wayfinder", "CONFIG.Token.objectClass.prototype.findMovementPath", function (wrapped, waypoints, options) {
        if (game.settings.get("wayfinder", "enablePathfinding")) {
            if (
                canvas.wayfinder &&
                canvas.scene &&
                !canvas.grid.isGridless &&
                !options?.constrainOptions?.ignoreWalls &&
                !options?.constrainOptions?.ignoreCost
            ) {
                let movementHistory = Array.isArray(options?.constrainOptions?.history)
                    ? options.constrainOptions.history
                    : options?.constrainOptions?.history
                      ? this.document.movementHistory
                      : [];

                let token = new CancellationToken();

                return {
                    result: undefined,
                    promise: canvas.wayfinder.findMovementPath(
                        token,
                        this.document,
                        waypoints,
                        game.settings.get("wayfinder", "fogExploration")
                            ? this.document.sight.enabled && canvas.fog.tokenVision && canvas.fog.fogExploration
                            : false,
                        this.document.measureMovementPath(movementHistory)
                    ),
                    cancel: () => {
                        token.cancel();
                    }
                };
            }
        }

        return wrapped(waypoints, options);
    });

    canvas.fog.addEventListener("explored", function () {
        canvas.wayfinder?.updateFog();
    });
});

Hooks.on("getSceneControlButtons", (controls) => {
    controls.tokens.tools.pathfinding = {
        name: "pathfinding",
        order: 3,
        title: "wayfinder.controls.pathfinding.title",
        icon: "fa-duotone fa-solid fa-compass",
        toggle: true,
        active: game.settings.get("wayfinder", "enablePathfinding"),
        toolclip: {
            src: "modules/wayfinder/toolclips/pathfinding.webm",
            heading: "wayfinder.controls.pathfinding.title",
            items: foundry.applications.ui.SceneControls.buildToolclipItems([{ paragraph: "wayfinder.controls.pathfinding.paragraph" }])
        },
        onChange(_event, active) {
            if (active !== undefined) game.settings.set("wayfinder", "enablePathfinding", active);
        }
    };
});

Hooks.on("canvasReady", (canvas) => {
    if (canvas.scene) {
        canvas.wayfinder = new Wayfinder();
    }
});

Hooks.on("canvasTearDown", (canvas) => {
    canvas.wayfinder?.free();
    canvas.wayfinder = undefined;
});

Hooks.on("createRegion", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.addRegion(document);
    }
});

Hooks.on("deleteRegion", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.deleteRegion(document);
    }
});

Hooks.on("updateRegion", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.updateRegion(document);
    }
});

Hooks.on("createWall", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.addWall(document);
    }
});

Hooks.on("deleteWall", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.deleteWall(document);
    }
});

Hooks.on("updateWall", (document) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.updateWall(document);
    }
});
