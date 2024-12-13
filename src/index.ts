import { ActorPF2e, CombatantPF2e, EncounterPF2e, EncounterTrackerPF2e, RulerPF2e, TokenPF2e } from "foundry-pf2e";
import init, { Wayfinder } from "wayfinder-crate";

declare global {
    interface ClientSettings {
        get(module: "wayfinder", setting: "enablePathfinding"): boolean;
        get(module: "wayfinder", setting: "fogExploration"): boolean;
        get(module: "wayfinder", setting: "enableMovementHistory"): boolean;
        get(module: "wayfinder", setting: "enableDifficultTerrain"): boolean;
        get(module: "wayfinder", setting: "enableActionIcons"): boolean;

        set(module: "wayfinder", setting: "enablePathfinding", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "fogExploration", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "enableMovementHistory", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "enableDifficultTerrain", value: boolean): Promise<boolean>;
        set(module: "wayfinder", setting: "enableActionIcons", value: boolean): Promise<boolean>;
    }

    interface TokenDocument {
        getFlag(scope: "wayfinder", key: "movementHistory"): WayfinderMovementHistory | undefined;
        setFlag(scope: "wayfinder", key: "movementHistory", value: WayfinderMovementHistory): Promise<this>;
        unsetFlag(scope: "wayfinder", key: "movementHistory"): Promise<this>;
    }

    interface Canvas {
        wayfinder?: Wayfinder;
    }
}

interface WayfinderMovementHistory {
    combatId: string;
    history: RulerMeasurementHistoryWaypoint[];
}

type WayfinderPoint = Point & {
    path: Point[];
};

function isWayfinderPoint(point: Point | null): point is WayfinderPoint {
    game.combat?.id;
    if (point === null) return false;
    return (point as WayfinderPoint).path !== undefined;
}

function getPath({
    history,
    waypoints,
    destination,
}: {
    history?: RulerMeasurementHistoryWaypoint[];
    waypoints?: Point[];
    destination?: Maybe<Point>;
} = {}) {
    const path = history
        ? history.map((p) => {
              return { x: p.x, y: p.y };
          })
        : [];

    if (waypoints) {
        path.push(...waypoints);
    }

    if (destination) {
        if (isWayfinderPoint(destination)) {
            path.push(...destination.path);
        } else {
            path.push(destination);
        }
    }

    return path;
}

function getActionSymbols(count: number): string {
    let symbols = [];

    if (!isFinite(count)) return "";

    while (count > 0) {
        symbols.push((count > 6 ? Array(6) : Array(count)).fill("◆").join(" "));
        count -= 6;
    }

    return "\r\n" + symbols.join("\r\n");
}

function getString(value: number): string {
    return `${Math.round(value * 100) / 100}`;
}

Hooks.once("init", async () => {
    CONFIG.fontDefinitions["Wayfinder Icons"] = {
        editor: false,
        fonts: [
            {
                urls: ["modules/wayfinder/fonts/wayfinder-icons.woff2"],
            },
        ],
    };

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

    game.settings.register("wayfinder", "enableMovementHistory", {
        name: "wayfinder.settings.enableMovementHistory.name",
        hint: "wayfinder.settings.enableMovementHistory.hint",
        scope: "world",
        config: true,
        requiresReload: true,
        type: Boolean,
        default: true,
    });

    game.settings.register("wayfinder", "enableDifficultTerrain", {
        name: "wayfinder.settings.enableDifficultTerrain.name",
        hint: "wayfinder.settings.enableDifficultTerrain.hint",
        scope: "world",
        config: true,
        requiresReload: true,
        type: Boolean,
        default: true,
    });

    game.settings.register("wayfinder", "enableActionIcons", {
        name: "wayfinder.settings.enableActionIcons.name",
        hint: "wayfinder.settings.enableActionIcons.hint",
        scope: "client",
        config: true,
        requiresReload: false,
        type: Boolean,
        default: true,
    });

    await init();
});

Hooks.once("ready", () => {
    if (CONFIG.Canvas.rulerClass.name !== "RulerPF2e") {
        ui.notifications.error("Wayfinder has been disabled because RulerPF2e is not the default ruler!");
        return;
    }

    libWrapper.register<RulerPF2e, RulerPF2e["_startMeasurement"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._startMeasurement",
        function (this: RulerPF2e, wrapped: RulerPF2e["_startMeasurement"], origin: Point, { snap = true, token } = {}) {
            if (!this.isDragMeasuring) {
                wrapped(origin, { snap, token });
            }

            if (game.settings.get("wayfinder", "enablePathfinding") && token && canvas.wayfinder) {
                if (Math.max(token.document.width, 1) % 2 !== 1) {
                    if (canvas.grid.isSquare) {
                        canvas.wayfinder.addOffset({ x: canvas.grid.size / 2, y: canvas.grid.size / 2 });
                    }

                    if (canvas.grid.isHexagonal) {
                        if (canvas.grid.columns) {
                            canvas.wayfinder.addOffset({ x: canvas.grid.sizeX / 4, y: canvas.grid.sizeY / 2 });
                        } else {
                            canvas.wayfinder.addOffset({ x: canvas.grid.sizeX / 2, y: canvas.grid.sizeY / 4 });
                        }
                    }
                }

                if (canvas.scene?.fog.exploration && game.settings.get("wayfinder", "fogExploration")) {
                    if (!game.settings.get("pf2e", "gmVision") && token.document.sight.enabled) {
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

                        canvas.wayfinder.addExplored(
                            // @ts-expect-error
                            canvas.app.renderer.gl,
                            Object.values(renderTexture.baseTexture._glTextures)[0],
                            sceneRect
                        );
                    }
                }
            }

            wrapped(origin, { snap, token });
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_endMeasurement"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._endMeasurement",
        function (this: RulerPF2e, wrapped: RulerPF2e["_endMeasurement"]) {
            if (!this.isDragMeasuring) {
                return wrapped();
            }

            if (this.state !== Ruler.STATES.MEASURING) return;

            wrapped();
            canvas.wayfinder?.addOffset(undefined);
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_getMeasurementDestination"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementDestination",
        function (this: RulerPF2e, wrapped: RulerPF2e["_getMeasurementDestination"], point: Point, { snap = true } = {}) {
            if (!this.isDragMeasuring) {
                return wrapped(point, { snap });
            }

            let destination = wrapped(point, { snap });

            if (this.user == game.user && snap) {
                if (this.token && canvas.wayfinder && game.settings.get("wayfinder", "enablePathfinding")) {
                    let path = canvas.wayfinder.findPath(getPath({ history: this.history, waypoints: this.waypoints }), destination);
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

    libWrapper.register<RulerPF2e, RulerPF2e["_getMeasurementSegments"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementSegments",
        function (this: RulerPF2e, wrapped: RulerPF2e["_getMeasurementSegments"]) {
            if (!this.isDragMeasuring) {
                return wrapped();
            }

            const segments: RulerMeasurementSegment[] = [];
            const path = getPath({ history: this.history, waypoints: this.waypoints, destination: this.destination });

            for (let i = 1; i < path.length; i++) {
                const label =
                    (this.labels.children.at(i - 1) as PreciseText) ??
                    this.labels.addChild(
                        new PreciseText(
                            "",
                            new PIXI.TextStyle({
                                fontFamily: ["Wayfinder Icons", "Signika"],
                                fontSize: 36,
                                fill: "#FFFFFF",
                                stroke: "#111111",
                                strokeThickness: 1,
                                dropShadow: true,
                                dropShadowColor: "#000000",
                                dropShadowBlur: 2,
                                dropShadowAngle: 0,
                                dropShadowDistance: 0,
                                align: "center",
                                wordWrap: false,
                                padding: 1,
                            })
                        )
                    );
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

    libWrapper.register<RulerPF2e, RulerPF2e["_addWaypoint"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._addWaypoint",
        function (this: RulerPF2e, wrapped: RulerPF2e["_addWaypoint"], point: Point, { snap = true } = {}) {
            if (!this.isDragMeasuring) {
                return wrapped(point, { snap });
            }

            if (this.state !== Ruler.STATES.STARTING && this.state !== Ruler.STATES.MEASURING) return;
            const waypoint =
                this.state === Ruler.STATES.STARTING
                    ? this._getMeasurementOrigin(point, { snap })
                    : this._getMeasurementDestination(point, { snap });

            if (isWayfinderPoint(waypoint)) {
                this.waypoints.push(...waypoint.path);
            } else {
                this.waypoints.push(waypoint);
            }

            this._state = Ruler.STATES.MEASURING;
            const destination = this.destination ?? point;
            this.measure({ x: destination.x, y: destination.y }, { snap, force: true });
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_getMeasurementHistory"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementHistory",
        function (this: RulerPF2e, wrapped: RulerPF2e["_getMeasurementHistory"]) {
            if (!this.isDragMeasuring) {
                return wrapped();
            }

            if (this.token && game.combat?.started && game.settings.get("wayfinder", "enableMovementHistory")) {
                if (this.token.inCombat) {
                    return this.token.document.getFlag("wayfinder", "movementHistory")?.history;
                }
            }

            return undefined;
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_postMove"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._postMove",
        async function (this: RulerPF2e, wrapped: RulerPF2e["_postMove"], token: TokenPF2e | null) {
            if (!this.isDragMeasuring) {
                return wrapped(token);
            }

            if (game.combat?.started && game.settings.get("wayfinder", "enableMovementHistory")) {
                if (token?.document.inCombat) {
                    token.document.setFlag("wayfinder", "movementHistory", {
                        combatId: game.combat.id,
                        history: this._createMeasurementHistory(),
                    });
                }
            }
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_getSegmentLabel"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getSegmentLabel",
        function (this: RulerPF2e, wrapped: RulerPF2e["_getSegmentLabel"], segment: RulerMeasurementSegment) {
            if (!this.isDragMeasuring) {
                return wrapped(segment);
            }

            if (segment.teleport) return "";
            const units = canvas.grid.units;
            let label = !game.settings.get("wayfinder", "enableDifficultTerrain")
                ? getString(segment.distance)
                : segment.distance == segment.cost
                  ? getString(segment.distance)
                  : `${getString(segment.cost)} / ${getString(segment.distance)}`;
            if (units) label += ` ${units}`;

            if (segment.last) {
                if (game.settings.get("wayfinder", "enableDifficultTerrain") && segment.cumulativeDistance != segment.cumulativeCost) {
                    label = "⚠ " + label;
                }

                label += !game.settings.get("wayfinder", "enableDifficultTerrain")
                    ? ` [${getString(segment.cumulativeDistance)}`
                    : segment.cumulativeDistance == segment.cumulativeCost
                      ? ` [${getString(segment.cumulativeDistance)}`
                      : ` [${getString(segment.cumulativeCost)} / ${getString(segment.cumulativeDistance)}`;
                if (units) label += ` ${units}`;
                label += "]";

                const actor = this.token?.actor as ActorPF2e;

                if (game.settings.get("wayfinder", "enableActionIcons") && actor && actor.isOfType("creature")) {
                    if (canvas.grid.distance == 5 && canvas.grid.units == "ft") {
                        const actionCost = Math.ceil(
                            (game.settings.get("wayfinder", "enableDifficultTerrain")
                                ? segment.cumulativeCost
                                : segment.cumulativeDistance) / actor.system.attributes.speed.total
                        );
                        label += getActionSymbols(actionCost);
                    }
                }
            }

            return label;
        }
    );

    libWrapper.register<RulerPF2e, RulerPF2e["_getMeasurementData"]>(
        "wayfinder",
        "CONFIG.Canvas.rulerClass.prototype._getMeasurementData",
        function (this: RulerPF2e, wrapped: RulerPF2e["_getMeasurementData"]) {
            if (!this.isDragMeasuring) {
                return wrapped();
            }

            let destinationPath = getPath({ destination: this.destination });
            let destination = destinationPath.pop() ?? null;

            return foundry.utils.deepClone({
                state: this.state,
                token: this.token?.id ?? null,
                history: this.history,
                waypoints: [...this.waypoints, ...destinationPath],
                destination: destination,
            });
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

Hooks.on("pf2e.startTurn", (combatant: CombatantPF2e, _encounter: EncounterPF2e, _userId: string) => {
    if (game.settings.get("wayfinder", "enableMovementHistory")) {
        combatant.token?.unsetFlag("wayfinder", "movementHistory");
    }
});

Hooks.on("getCombatTrackerEntryContext", (_application: EncounterTrackerPF2e<EncounterPF2e>, entryOptions: ContextMenuEntry[]) => {
    if (game.settings.get("wayfinder", "enableMovementHistory")) {
        entryOptions.splice(1, 0, {
            name: "wayfinder.clearMovementHistory",
            icon: '<i class="fas fa-eraser"></i>',
            condition: (li) => {
                const combatant = game.combat?.combatants.get(li.data("combatant-id"));
                return combatant?.token?.getFlag("wayfinder", "movementHistory") != undefined;
            },
            callback: (li) => {
                const combatant = game.combat?.combatants.get(li.data("combatant-id"));
                if (combatant) combatant.token?.unsetFlag("wayfinder", "movementHistory");
            },
        });
    }
});

Hooks.on("canvasReady", (canvas: Canvas) => {
    if (canvas.scene) {
        canvas.wayfinder = new Wayfinder(
            canvas.scene.dimensions.sceneRect,
            canvas.grid,
            canvas.walls.placeables.map((w) => w.document)
        );
    }
});

Hooks.on("canvasTearDown", (canvas: Canvas) => {
    canvas.wayfinder?.free();
    canvas.wayfinder = undefined;
});

Hooks.on("createWall", (document: WallDocument<Scene>, _options: DatabaseCreateOperation<WallDocument<Scene>>, _userId: string) => {
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
});
