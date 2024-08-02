import { MODULE_ID } from "./const.js";

export function getPath(history = [], waypoints = [], destination = null) {
    let path = [];

    for (let element of history) {
        if ("path" in element) {
            if (path.length == 0) {
                path.push(...element.path);
            } else {
                path.push(...element.path.slice(1));
            }
        } else {
            path.push(element);
        }
    }

    for (let element of waypoints) {
        if ("path" in element) {
            if (path.length == 0) {
                path.push(...element.path);
            } else {
                path.push(...element.path.slice(1));
            }
        } else {
            path.push(element);
        }
    }

    if (destination) {
        if ("path" in destination) {
            path.push(...destination.path.slice(1));
        } else {
            path.push(destination);
        }
    }

    return path;
}