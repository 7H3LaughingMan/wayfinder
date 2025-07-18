import { TokenPF2e } from "foundry-pf2e";
import {
    TokenFindMovementPathOptions,
    TokenFindMovementPathWaypoint,
    TokenMeasuredMovementWaypoint,
} from "foundry-pf2e/foundry/client/_types.mjs";
import { Canvas } from "foundry-pf2e/foundry/client/canvas/_module.mjs";
import { WallDocument } from "foundry-pf2e/foundry/client/documents/_module.mjs";
import {
    DatabaseCreateOperation,
    DatabaseDeleteOperation,
    DatabaseUpdateOperation,
} from "foundry-pf2e/foundry/common/abstract/_module.mjs";
import init, { Wayfinder } from "../pkg/wayfinder.js";
import { GridOffset2D } from "foundry-pf2e/foundry/common/grid/_types.mjs";
import FogManager from "foundry-pf2e/foundry/client/canvas/perception/fog.mjs";
import { Point } from "foundry-pf2e/foundry/common/_types.mjs";
import CanvasVisibility from "foundry-pf2e/foundry/client/canvas/groups/visibility.mjs";

declare module "foundry-pf2e" {
    interface ClientSettingsPF2e {
        get(module: "wayfinder", settings: "enablePathfinding"): boolean;
        get(module: "wayfinder", settings: "fogExploration"): boolean;

        set(module: "wayfinder", setting: "enablePathfinding", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "fogExploration", value: boolean): Promise<boolean>;
    }
}

declare module "foundry-pf2e/foundry/client/canvas/_module.mjs" {
    interface Canvas {
        wayfinder?: Wayfinder;
    }
}

CONFIG.debug.fog.manager = true;

Hooks.once("init", async () => {
    game.settings.register("wayfinder", "enablePathfinding", {
        name: "enablePathfinding",
        scope: "user",
        config: false,
        type: Boolean,
        default: false,
    });

    game.settings.register("wayfinder", "fogExploration", {
        name: "wayfinder.settings.fogExploration.name",
        hint: "wayfinder.settings.fogExploration.hint",
        scope: "world",
        config: true,
        type: Boolean,
        default: true,
    });

    await init();
});

function updateExploration() {
    if (!canvas.wayfinder) {
        return;
    }

    if (!canvas.fog.tokenVision || !canvas.fog.fogExploration) {
        return;
    }

    let textureConfiguration = canvas.fog.textureConfiguration;
    let sprite = canvas.fog.sprite;

    let renderTexture = PIXI.RenderTexture.create({
        width: sprite.width * textureConfiguration.resolution,
        height: sprite.height * textureConfiguration.resolution,
    });
    let transform = new PIXI.Matrix(
        textureConfiguration.resolution,
        0,
        0,
        textureConfiguration.resolution,
        -(sprite.x * textureConfiguration.resolution),
        -(sprite.y * textureConfiguration.resolution)
    );
    canvas.app.renderer.render(sprite, { renderTexture, transform });

    canvas.wayfinder.updateFog(
        (canvas.app.renderer as PIXI.Renderer).gl,
        renderTexture.baseTexture._glTextures[(canvas.app.renderer as PIXI.Renderer).CONTEXT_UID],
        canvas.dimensions.sceneRect,
        textureConfiguration.resolution
    );
}

Hooks.once("ready", () => {
    libWrapper.register<TokenPF2e, TokenPF2e["findMovementPath"]>(
        "wayfinder",
        "CONFIG.Token.objectClass.prototype.findMovementPath",
        function (
            this: TokenPF2e,
            _wrapped: TokenPF2e["findMovementPath"],
            waypoints: TokenFindMovementPathWaypoint[],
            options: TokenFindMovementPathOptions
        ) {
            if (game.settings.get("wayfinder", "enablePathfinding")) {
                if (canvas.wayfinder && canvas.scene && !canvas.grid.isGridless && !options.ignoreWalls && !options.ignoreCost) {
                    let movementHistory: TokenMeasuredMovementWaypoint[] = Array.isArray(options.history)
                        ? options.history
                        : options.history
                          ? this.document.movementHistory
                          : [];

                    return {
                        result: undefined,
                        promise: canvas.wayfinder?.findMovementPath(
                            this.document,
                            waypoints,
                            game.settings.get("wayfinder", "fogExploration")
                                ? !(game.user.isGM && (!this.document.sight.enabled || game.system.id !== "pf2e" || game.settings.get("pf2e", "gmVision"))) &&
                                      canvas.scene.tokenVision &&
                                      canvas.scene.fog.exploration
                                : false,
                            this.document.measureMovementPath(movementHistory)
                        ),
                        cancel: () => {},
                    };
                }
            }

            const [path] = this.constrainMovementPath(waypoints, options);
            return { result: path, promise: Promise.resolve(path), cancel: () => {} };
        }
    );

    canvas.fog.addEventListener("explored", function (event: Event) {
        updateExploration();
    });
});

Hooks.on("getSceneControlButtons", (controls) => {
    const tokenTools = controls.tokens?.tools;
    if (tokenTools) {
        tokenTools.pathfinding = {
            name: "pathfinding",
            order: 3,
            title: "wayfinder.controls.pathfinding.title",
            icon: "fa-duotone fa-solid fa-compass",
            toggle: true,
            active: game.settings.get("wayfinder", "enablePathfinding"),
            toolclip: {
                src: "modules/wayfinder/toolclips/pathfinding.webm",
                heading: "wayfinder.controls.pathfinding.title",
                items: foundry.applications.ui.SceneControls.buildToolclipItems([
                    { paragraph: "wayfinder.controls.pathfinding.paragraph" },
                ]),
            },
            onChange(_event, active) {
                if (active !== undefined) game.settings.set("wayfinder", "enablePathfinding", active);
            },
        };
    }
});

Hooks.on("canvasReady", (canvas: Canvas) => {
    if (canvas.scene) {
        canvas.wayfinder = new Wayfinder(
            canvas.scene.dimensions.sceneRect,
            canvas.grid,
            canvas.walls.placeables.map((w) => w.document)
        );

        updateExploration();
    }
});

Hooks.on("canvasTearDown", (canvas: Canvas) => {
    canvas.wayfinder?.free();
    canvas.wayfinder = undefined;
});

Hooks.on("createWall", (document: WallDocument, _options: DatabaseCreateOperation<WallDocument<Scene>>, _userId: string) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.addWall(document);
    }
});

Hooks.on(
    "updateWall",
    (document: WallDocument<Scene>, _change: object, _options: DatabaseUpdateOperation<WallDocument<Scene>>, _userId: string) => {
        if (document.parent == game.scenes.current) {
            canvas.wayfinder?.updateWall(document);
        }
    }
);

Hooks.on("deleteWall", (document: WallDocument<Scene>, _options: DatabaseDeleteOperation<WallDocument<Scene>>, _userId: string) => {
    if (document.parent == game.scenes.current) {
        canvas.wayfinder?.deleteWall(document);
    }
    foundry.documents.TokenDocument;
});
