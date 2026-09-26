use crate::{
    CANVAS,
    exports::CancellationToken,
    log,
    types::{
        foundry::{
            JsPoint,
            documents::{
                JsPartialTokenMovementWaypoint, JsRegionDocument, JsTokenDocument, JsTokenMovementWaypoint,
                JsWallDocument,
            },
            grid::JsGridMeasurePathResult,
        },
        helpers::JsObject,
        wayfinder::{
            FogManager, GridMeasurePathResult, PartialTokenMovementWaypoint, RegionManager, Scene, TokenDocument,
            TokenMovementWaypoint, WallManager, grid::Grid,
        },
    },
};
use geo::Rect;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Wayfinder {
    scene_rect: Rect,
    fog_manager: FogManager,
    grid: Grid,
    region_manager: RegionManager,
    wall_manager: WallManager,
}

#[wasm_bindgen]
impl Wayfinder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Wayfinder {
        let scene_rect = CANVAS.scene().unwrap().dimensions().scene_rect().into();
        let grid = Grid::new();
        let region_documents =
            CANVAS.scene().unwrap().regions().values().into_iter().flatten().map(JsRegionDocument::into).collect();
        let wall_documents =
            CANVAS.scene().unwrap().walls().values().into_iter().flatten().map(JsWallDocument::into).collect();

        Wayfinder {
            scene_rect,
            fog_manager: FogManager::new(),
            grid,
            region_manager: RegionManager::new(region_documents),
            wall_manager: WallManager::new(wall_documents),
        }
    }

    #[wasm_bindgen]
    pub fn debug(&self) {
        log!("{self:#?}");
    }

    #[wasm_bindgen(js_name = isPointExplored)]
    pub fn is_point_explored(&mut self, point: JsPoint) -> bool {
        self.fog_manager.is_point_explored(point.into())
    }

    #[wasm_bindgen(js_name = updateFog)]
    pub fn update_fog(&mut self) {
        self.fog_manager = FogManager::new();
    }

    #[wasm_bindgen(js_name = generateMaze)]
    pub fn generate_maze(&self, width: usize, height: usize) -> JsObject {
        Scene::new(width.min(500), height.min(500)).to_object()
    }

    #[wasm_bindgen(js_name = addRegion)]
    pub fn add_region(&mut self, region_document: JsRegionDocument) {
        self.region_manager.add_region(region_document.into())
    }

    #[wasm_bindgen(js_name = deleteRegion)]
    pub fn delete_region(&mut self, region_document: JsRegionDocument) {
        self.region_manager.delete_region(region_document.into());
    }

    #[wasm_bindgen(js_name = updateRegion)]
    pub fn update_region(&mut self, region_document: JsRegionDocument) {
        self.region_manager.update_region(region_document.into());
    }

    #[wasm_bindgen(js_name = addWall)]
    pub fn add_wall(&mut self, wall_document: JsWallDocument) {
        self.wall_manager.add_wall(wall_document.into())
    }

    #[wasm_bindgen(js_name = deleteWall)]
    pub fn delete_wall(&mut self, wall_document: JsWallDocument) {
        self.wall_manager.delete_wall(wall_document.into());
    }

    #[wasm_bindgen(js_name = updateWall)]
    pub fn update_wall(&mut self, wall_document: JsWallDocument) {
        self.wall_manager.update_wall(wall_document.into());
    }

    #[wasm_bindgen(js_name = findMovementPath, unchecked_return_type = "TokenMovementWaypoint[] | null")]
    pub async fn find_movement_path(
        &self,
        cancellation_token: &CancellationToken,
        token_document: JsTokenDocument,
        waypoints: Vec<JsPartialTokenMovementWaypoint>,
        use_exploration: bool,
        grid_measure_path_result: JsGridMeasurePathResult,
    ) -> JsValue {
        let token_document: TokenDocument = token_document.into();
        let waypoints: Vec<PartialTokenMovementWaypoint> =
            waypoints.into_iter().map(PartialTokenMovementWaypoint::from).collect();
        let grid_measure_path_result: GridMeasurePathResult = grid_measure_path_result.into();

        let mut new_waypoints: Vec<TokenMovementWaypoint> = Vec::new();
        let mut default_waypoint = &token_document.create_waypoint();
        for waypoint in waypoints {
            new_waypoints.push(waypoint.create_waypoint(default_waypoint));
            default_waypoint = new_waypoints.last().unwrap();
        }

        let path = self.grid.find_path(
            cancellation_token,
            new_waypoints,
            &token_document,
            &self.scene_rect,
            if use_exploration { Some(&self.fog_manager) } else { None },
            &self.region_manager,
            &self.wall_manager,
            &grid_measure_path_result,
        );

        match cancellation_token.status() {
            false => path.into_iter().map(JsTokenMovementWaypoint::from).collect::<Array>().into(),
            true => JsValue::null(),
        }
    }
}
