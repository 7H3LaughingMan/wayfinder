import init, { Wayfinder } from "wayfinder-crate";

declare global {
    interface ClientSettings {
        get(module: "wayfinder", setting: "enablePathfinding"): boolean;
        get(module: "wayfinder", setting: "fogExploration"): boolean;
        //get(module: "wayfinder", setting: "enableMovementHistory"): boolean;

        set(module: "wayfinder", setting: "enablePathfinding", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "fogExploration", value: boolean): Promise<boolean>;
        //set(module: "wayfinder", setting: "enableMovementHistory", value: boolean): Promise<boolean>;
    }

    interface TokenDocument {
        //getFlag(scope: "wayfinder", key: "movementHistory"): WayfinderMovementHistory | undefined;
    }

    interface Ruler {
        wayfinder?: Wayfinder;
    }
}

//interface WayfinderMovementHistory {}

type WayfinderPoint = Point & {
    path: Point[];
};

function isWayfinderPoint(point: Point | null): point is WayfinderPoint {
    if (point === null) return false;
    return (point as WayfinderPoint).path !== undefined;
}

function getPath(history: RulerMeasurementHistoryWaypoint[], waypoints: Point[], destination?: Point | null) {
    const path = (history as Point[]).concat(waypoints);

    if (destination) {
        if (isWayfinderPoint(destination)) {
            path.push(...destination.path);
        } else {
            path.push(destination);
        }
    }

    return path;
}

Hooks.once("init", async () => {
    game.settings.register("wayfinder", "enablePathfinding", {
        name: "enablePathfinding",
        scope: "client",
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

    /*game.settings.register("wayfinder", "enableMovementHistory", {
        name: "wayfinder.settings.enableMovementHistory.name",
        hint: "wayfinder.settings.enableMovementHistory.hint",
        scope: "world",
        config: true,
        requiresReload: true,
        type: Boolean,
        default: true,
    });*/

    await init();
});

Hooks.once("ready", () => {
    if (CONFIG.Canvas.rulerClass.name !== "RulerPF2e") {
        ui.notifications.error("Wayfinder has been disabled because RulerPF2e is not the default ruler!");
        return;
    }

    libWrapper.register<Ruler, Ruler["_startMeasurement"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._startMeasurement",
        function (this: Ruler, wrapped: Ruler["_startMeasurement"], ...args: Parameters<Ruler["_startMeasurement"]>) {
            if (this.state !== Ruler.STATES.INACTIVE) return;

            if (game.settings.get("wayfinder", "enablePathfinding") && args[1]?.token) {
                this.wayfinder = new Wayfinder();

                this.wayfinder.addGrid(canvas.grid);
                this.wayfinder.addToken(args[1].token);
                this.wayfinder.addBounds(canvas.scene?.dimensions.sceneRect);
                this.wayfinder.addWalls(canvas.walls.placeables.map((w) => w.document));

                if (canvas.scene?.fog.exploration && game.settings.get("wayfinder", "fogExploration")) {
                    if (!game.settings.get("pf2e", "gmVision") && args[1].token.document.sight.enabled) {
                        let scale = 0.1;
                        let sceneRect = canvas.dimensions.sceneRect;
                        let scaledRect = new PIXI.Rectangle(
                            sceneRect.x * scale,
                            sceneRect.y * scale,
                            sceneRect.width * scale,
                            sceneRect.height * scale
                        );

                        let renderTexture = PIXI.RenderTexture.create({ width: scaledRect.width, height: scaledRect.height });
                        let transform = new PIXI.Matrix(scale, 0, 0, scale, -scaledRect.x, -scaledRect.y);
                        canvas.app.renderer.render(canvas.visibility.explored, { renderTexture, transform });

                        let explored_pixels = canvas.app.renderer.extract.pixels(renderTexture) as Uint8Array;

                        this.wayfinder.addExplored(
                            explored_pixels,
                            canvas.dimensions.sceneRect,
                            new PIXI.Rectangle(0, 0, renderTexture.width, renderTexture.height)
                        );
                    }
                }
            }

            wrapped(...args);
        }
    );

    libWrapper.register<Ruler, Ruler["_endMeasurement"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._endMeasurement",
        function (this: Ruler, wrapped: Ruler["_endMeasurement"]) {
            if (this.state !== Ruler.STATES.MEASURING) return;

            wrapped();
            this.wayfinder?.free();
            this.wayfinder = undefined;
        }
    );

    libWrapper.register<Ruler, Ruler["_getMeasurementDestination"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementDestination",
        function (this: Ruler, wrapped: Ruler["_getMeasurementDestination"], ...args: Parameters<Ruler["_getMeasurementDestination"]>) {
            let destination = wrapped(...args);

            if (this.user == game.user && args[1]?.snap) {
                if (this.token && this.wayfinder && game.settings.get("wayfinder", "enablePathfinding")) {
                    let path = this.wayfinder.findPath(getPath(this.history, this.waypoints), destination);
                    let mode =
                        Math.max(this.token.document.width, 1) % 2 === 1
                            ? CONST.GRID_SNAPPING_MODES.CENTER
                            : CONST.GRID_SNAPPING_MODES.BOTTOM_RIGHT_VERTEX;

                    if (path && path.length > 1) {
                        return {
                            x: destination.x,
                            y: destination.y,
                            path: path.map((point) => canvas.grid.getSnappedPoint(point, { mode })),
                        };
                    }
                }
            }

            return destination;
        }
    );

    libWrapper.register<Ruler, Ruler["_getMeasurementSegments"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementSegments",
        function (this: Ruler, _wrapped: Ruler["_getMeasurementSegments"]) {
            const segments: RulerMeasurementSegment[] = [];
            const path = getPath(this.history, this.waypoints, this.destination);

            for (let i = 1; i < path.length; i++) {
                const label =
                    (this.labels.children.at(i - 1) as PreciseText) ?? this.labels.addChild(new PreciseText("", CONFIG.canvasTextStyle));
                const ray = new Ray(path[i - 1], path[i]);
                segments.push({
                    ray,
                    teleport: i < this.history.length ? this.history[i].teleport : i === this.history.length && ray.distance > 0,
                    label,
                    distance: 0,
                    cost: 0,
                    cumulativeDistance: 0,
                    cumulativeCost: 0,
                    history: i <= this.history.length,
                    first: i === this.history.length + 1,
                    last: i === path.length - 1,
                    // @ts-expect-error
                    animation: {},
                });
            }

            if (this.labels.children.length > segments.length) {
                this.labels.removeChildren(segments.length).forEach((c) => c.destroy());
            }

            return segments;
        }
    );

    libWrapper.register<Ruler, Ruler["_addWaypoint"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._addWaypoint",
        function (this: Ruler, _wrapped: Ruler["_addWaypoint"], ...args: Parameters<Ruler["_addWaypoint"]>) {
            if (this.state !== Ruler.STATES.STARTING && this.state !== Ruler.STATES.MEASURING) return;
            const waypoint =
                this.state === Ruler.STATES.STARTING
                    ? this._getMeasurementOrigin(args[0], { snap: args[1]?.snap })
                    : this._getMeasurementDestination(args[0], { snap: args[1]?.snap });

            if (isWayfinderPoint(waypoint)) {
                this.waypoints.push(...waypoint.path);
            } else {
                this.waypoints.push(waypoint);
            }

            this._state = Ruler.STATES.MEASURING;
            const destination = this.destination ?? args[0];
            this.measure({ x: destination.x, y: destination.y }, { snap: args[1]?.snap, force: true });
        }
    );
});

Hooks.on("getSceneControlButtons", (controls: SceneControl[]) => {
    if (!canvas.scene) return;

    const tokenControls = controls.find((c) => c.name === "token");
    const rulerIndex = tokenControls?.tools.findIndex((t) => t.name === "ruler");

    if (rulerIndex) {
        tokenControls?.tools.splice(rulerIndex + 1, 0, {
            visible: true,
            name: "pathfinding",
            title: "wayfinder.controls.pathfinding",
            icon: "fa-duotone fa-solid fa-compass",
            toggle: true,
            active: game.settings.get("wayfinder", "enablePathfinding"),
            onClick: () => {
                const newStatus = !game.settings.get("wayfinder", "enablePathfinding");
                game.settings.set("wayfinder", "enablePathfinding", newStatus);
            },
        });
    }
});
