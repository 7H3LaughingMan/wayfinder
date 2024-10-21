use crate::{
    exports::JsWallDocument,
    traits::JsDeserialize,
    types::{Point, Wall},
};
use rapier2d::{parry::query::ShapeCastOptions, prelude::*};
use std::collections::HashMap;

pub struct Walls {
    colliders: ColliderSet,
    pub walls: HashMap<ColliderHandle, Wall>,
    pipeline: QueryPipeline,
    rigid_bodies: RigidBodySet,
}

impl Walls {
    pub fn new(wall_documents: Vec<JsWallDocument>) -> Self {
        let mut colliders = ColliderSet::new();
        let mut walls = HashMap::new();

        for wall in wall_documents {
            let wall: Wall = JsDeserialize::from_value(wall.into());

            if wall.blocks_movement() {
                let handle = colliders.insert(wall);
                walls.insert(handle, wall);
            }
        }

        let mut pipeline = QueryPipeline::new();
        pipeline.update(&colliders);

        Walls { colliders, walls, pipeline, rigid_bodies: RigidBodySet::new() }
    }

    pub fn check_collision(&self, start: Point, end: Point, shape: &Polyline) -> bool {
        let shape_pos = Isometry::new(vector![start.x, start.y], 0.0);
        let shape_vel = vector![end.x - start.x, end.y - start.y];
        let options = ShapeCastOptions {
            max_time_of_impact: 1.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        };

        if let Some((_handle, _toi)) = self.pipeline.cast_shape(
            &self.rigid_bodies,
            &self.colliders,
            &shape_pos,
            &shape_vel,
            shape,
            options,
            QueryFilter::default(),
        ) {
            return true;
        }

        false
    }
}
