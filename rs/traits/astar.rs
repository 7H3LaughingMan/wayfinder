use std::collections::HashMap;

use crate::{
    exports::Walls,
    types::{GridMeasurePathResult, GridOffset2D, Point, Rectangle, TokenDocument, TokenMovementWaypoint},
};

pub trait AStar {
    fn find_path(
        &self,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<HashMap<GridOffset2D, bool>>,
        grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint>;
}
