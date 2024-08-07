import { MODULE_ID } from "./const.js";
import { Wayfinder } from "../wasm/pf2e-astar.js"
import { getPath } from "./util.js";

export function wrapRuler() {
    if (CONFIG.Canvas.rulerClass.name !== "RulerPF2e") {
        ui.notifications.error("Wayfinder has been disabled because RulerPF2e is not the default ruler!");
        return;
    }

    libWrapper.register(MODULE_ID, "CONFIG.Canvas.rulerClass.prototype._startMeasurement", function (wrapped, origin, { snap = true, token } = {}) {
        if (this.state !== Ruler.STATES.INACTIVE) return;

        if (game.settings.get(MODULE_ID, "enablePathfinding") && token) {
            this.wayfinder = new Wayfinder(token);

            if (canvas.scene.fog.exploration && game.settings.get(MODULE_ID, "fogExploration")) {
                if (!game.settings.get("pf2e", "gmVision") && token.document.sight.enabled) {
                    let sceneRect = canvas.dimensions.sceneRect;
                    let scaledRect = new PIXI.Rectangle(sceneRect.x * 0.05, sceneRect.y * 0.05, sceneRect.width * 0.05, sceneRect.height * 0.05);

                    let renderTexture = PIXI.RenderTexture.create({ width: scaledRect.width, height: scaledRect.height });
                    let transform = new PIXI.Matrix(0.05, 0, 0, 0.05, -(scaledRect.x), -(scaledRect.y));
                    canvas.app.renderer.render(canvas.visibility.explored, { renderTexture, transform });

                    let explored_pixels = canvas.app.renderer.extract.pixels(renderTexture);
                    let explored_bounds = canvas.visibility.explored.getLocalBounds();

                    this.wayfinder.addExplored(explored_pixels, explored_bounds, scaledRect);
                }
            }
        }

        wrapped(origin, { snap, token });
    });

    libWrapper.register(MODULE_ID, "CONFIG.Canvas.rulerClass.prototype._endMeasurement", function (wrapped) {
        if (this.state !== Ruler.STATES.MEASURING) return;

        this.wayfinder = null;
        wrapped();
    });

    libWrapper.register(MODULE_ID, "CONFIG.Canvas.rulerClass.prototype._getMeasurementDestination", function (wrapped, point, { snap = true } = {}) {
        let destination = wrapped(point, { snap });

        if (this.user == game.user) {
            if (this.token && this.wayfinder && game.settings.get(MODULE_ID, "enablePathfinding")) {
                let path = this.wayfinder.findPath(getPath(this.history, this.waypoints), destination);

                if (path && path.length > 1) {
                    destination.path = path;
                } else {
                    delete destination.path;
                }
            }
        } else {
            if ("path" in point) {
                destination.path = point.path;
            }
        }

        return destination;
    });

    libWrapper.register(MODULE_ID, "CONFIG.Canvas.rulerClass.prototype._getMeasurementSegments", function (wrapped) {
        const segments = [];
        const path = getPath(this.history, this.waypoints, this.destination);

        for (let i = 1; i < path.length; i++) {
            const label = this.labels.children.at(i - 1) ?? this.labels.addChild(new PreciseText("", CONFIG.canvasTextStyle));
            const history = i < this.history.length;
            const first = i === this.history.length;
            const ray = new Ray(path[i - 1], path[i]);
            segments.push({
                ray,
                teleport: history ? path[i].teleport : first && (i > 0) && (ray.distance > 0),
                label,
                distance: 0,
                cost: 0,
                cumulativeDistance: 0,
                cumulativeCost: 0,
                history,
                first,
                last: i === path.length - 1,
                animation: {}
            });
        }

        if (this.labels.children.length > segments.length) {
            this.labels.removeChildren(segments.length).forEach(c => c.destroy());
        }

        return segments;
    });
}