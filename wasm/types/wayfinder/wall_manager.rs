use crate::types::wayfinder::{ByAddress, WallDocument};
use geo::{BoundingRect, Coord, Intersects, Line, Rect};
use rstar::{AABB, RTree};
use std::collections::HashMap;

#[derive(Debug)]
pub struct WallManager {
    tree: RTree<ByAddress<WallDocument>>,
    map: HashMap<String, ByAddress<WallDocument>>,
}

impl WallManager {
    pub fn new(wall_documents: Vec<WallDocument>) -> Self {
        let mut tree = RTree::new();
        let mut map = HashMap::new();

        for wall in wall_documents {
            let wall = ByAddress::new(wall);

            map.insert(wall.borrow().id.clone(), wall.clone());
            tree.insert(wall.clone());
        }

        WallManager { tree, map }
    }

    pub fn add_wall(&mut self, wall_document: WallDocument) {
        let wall = ByAddress::new(wall_document);

        self.map.insert(wall.borrow().id.clone(), wall.clone());
        self.tree.insert(wall);
    }

    pub fn delete_wall(&mut self, wall_document: WallDocument) {
        let handle = self.map.remove(&wall_document.id);

        if let Some(target) = handle {
            self.tree.remove(&target);
        }
    }

    pub fn update_wall(&mut self, wall_document: WallDocument) {
        self.delete_wall(wall_document.clone());
        self.add_wall(wall_document.clone());
    }

    pub fn get_walls(&self, bounds: Rect) -> Vec<WallDocument> {
        self.tree
            .locate_in_envelope_intersecting(&AABB::from_corners(bounds.min(), bounds.max()))
            .map(|wall| wall.borrow().clone())
            .collect()
    }

    pub fn check_collision(&self, start: Coord, end: Coord) -> bool {
        let line = Line::new(start, end);
        let bounds = line.bounding_rect();

        for wall in self.tree.locate_in_envelope_intersecting(&AABB::from_corners(bounds.min(), bounds.max())) {
            if wall.borrow().blocks_movement() {
                if wall.intersects(&line) {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_collisions(&self, lines: Vec<(Coord, Coord)>) -> bool {
        for (start, end) in lines {
            if self.check_collision(start, end) {
                return true;
            }
        }

        false
    }
}
