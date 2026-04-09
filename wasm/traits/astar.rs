use crate::{
    exports::CancellationToken,
    traits::{BaseGrid, Node, TokenShape},
    types::wayfinder::{
        FogManager, GridMeasurePathResult, RegionManager, TokenDocument, TokenMovementWaypoint, WallManager,
    },
};
use geo::Rect;

pub trait AStar<N: Node + Eq, T: TokenShape>: BaseGrid<N, T> {
    fn find_path(
        &self,
        cancellation_token: &CancellationToken,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        scene_rect: &Rect,
        fog_manager: Option<&FogManager>,
        region_manager: &RegionManager,
        wall_manager: &WallManager,
        grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint>;
}
