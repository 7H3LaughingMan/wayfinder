use crate::{
    enums::Grid,
    exports::JsWallDocument,
    modules::geometry,
    traits::{JsDeserialize, JsDeserializeVector, JsHelper},
    types::{ElevatedPoint, GridOffset3D, Point, QuadPointer, Quadtree, QuadtreeObject, Ray, Rectangle, Wall},
};
use pathfinding::prelude::Edge;
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};
use wasm_bindgen::{convert::IntoWasmAbi, JsValue};

pub struct Walls {
    walls: QuadPointer<Quadtree<Wall>>,
    map: HashMap<String, QuadPointer<Wall>>,
}

impl Walls {
    pub fn new(bounds: Rectangle, wall_documents: Vec<JsWallDocument>) -> Self {
        let walls = Quadtree::new(bounds, None, None, None, None);
        let mut map = HashMap::new();

        let wall_documents = Wall::from_js_vector(wall_documents);

        for wall in wall_documents {
            let wall = Rc::new(RefCell::new(wall));

            map.insert(wall.borrow().id.clone(), wall.clone());
            walls.borrow_mut().insert(QuadtreeObject::new(wall.borrow().get_bounds(), wall.clone()));
        }

        Walls { walls, map }
    }

    pub fn add_wall(&mut self, wall_document: JsWallDocument) {
        let wall = Rc::new(RefCell::new(Wall::from_js(wall_document)));

        self.map.insert(wall.borrow().id.clone(), wall.clone());
        self.walls.borrow_mut().insert(QuadtreeObject::new(wall.borrow().get_bounds(), wall.clone()));
    }

    pub fn delete_wall(&mut self, wall_document: JsWallDocument) {
        let wall = Wall::from_js(wall_document);
        let handle = self.map.remove(&wall.id);

        if let Some(h) = handle {
            self.walls.borrow_mut().remove(h);
        }
    }

    pub fn update_wall(&mut self, wall_document: JsWallDocument) {
        self.delete_wall(wall_document.clone().into());
        self.add_wall(wall_document.clone().into());
    }

    pub fn check_collision(&self, start: Point, end: Point) -> bool {
        let ray = Ray::new(start, end);
        let edges = self.walls.borrow().get_objects(ray.bounds());

        for edge in edges {
            if edge.borrow().blocks_movement() {
                if geometry::line_segment_intersection(
                    &ray.a,
                    &ray.b,
                    &edge.borrow().get_a(),
                    &edge.borrow().get_b(),
                    None,
                )
                .is_some()
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_collisions(&self, offsets: Vec<(Point, Point)>) -> bool {
        for (start, end) in offsets {
            if self.check_collision(start, end) {
                return true;
            }
        }

        false
    }

    pub fn check_point(&self, point: Point) -> bool {
        let edges = self.walls.borrow().get_objects(Rectangle { x: point.x, y: point.y, width: 0.0, height: 0.0 });

        for edge in edges {
            let edge = edge.borrow();
            if edge.blocks_movement() {
                if let Some(closest_point) = geometry::closest_point_to_segment(&point, &edge.get_a(), &edge.get_b()) {
                    if point.close(closest_point) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
