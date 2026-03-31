use crate::types::{ByAddress, ElevatedPoint, RegionDocument};
use geo::Contains;
use rstar::{AABB, RTree};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RegionManager {
    tree: RTree<ByAddress<RegionDocument>>,
    map: HashMap<String, ByAddress<RegionDocument>>,
}

impl RegionManager {
    pub fn new(wall_documents: Vec<RegionDocument>) -> Self {
        let mut tree = RTree::new();
        let mut map = HashMap::new();

        for wall in wall_documents {
            let wall = ByAddress::new(wall);

            map.insert(wall.borrow().id.clone(), wall.clone());
            tree.insert(wall.clone());
        }

        RegionManager { tree, map }
    }

    pub fn add_region(&mut self, region_document: RegionDocument) {
        let wall = ByAddress::new(region_document);

        self.map.insert(wall.borrow().id.clone(), wall.clone());
        self.tree.insert(wall);
    }

    pub fn delete_region(&mut self, region_document: RegionDocument) {
        let handle = self.map.remove(&region_document.id);

        if let Some(target) = handle {
            self.tree.remove(&target);
        }
    }

    pub fn update_region(&mut self, region_document: RegionDocument) {
        self.delete_region(region_document.clone());
        self.add_region(region_document.clone());
    }

    pub fn get_regions(&self, point: ElevatedPoint) -> Vec<RegionDocument> {
        self.tree
            .locate_in_envelope_intersecting(&AABB::from_point(point.into()))
            .filter(|region| region.borrow().contains(&point))
            .map(|region| region.borrow().clone())
            .collect()
    }
}
