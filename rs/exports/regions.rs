use crate::{
    exports::wayfinder::JsRegionDocumentSource,
    traits::{JsDeserialize, JsDeserializeVector, JsHelper},
    types::Region,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Regions {
    map: HashMap<String, Rc<RefCell<Region>>>,
}

impl Regions {
    pub fn new(region_documents: Vec<JsRegionDocumentSource>) -> Self {
        let mut map = HashMap::new();
        let region_documents = Region::from_js_vector(region_documents);

        for region in region_documents {
            let region = Rc::new(RefCell::new(region));
            map.insert(region.borrow().id.clone(), region.clone());
        }

        Regions { map }
    }

    pub fn add_region(&mut self, region_document: JsRegionDocumentSource) {
        let region = Rc::new(RefCell::new(Region::from_js(region_document)));
        self.map.insert(region.borrow().id.clone(), region.clone());
    }

    pub fn delete_region(&mut self, region_document: JsRegionDocumentSource) {
        let region = Region::from_js(region_document);
        self.map.remove(&region.id);
    }

    pub fn update_region(&mut self, region_document: JsRegionDocumentSource) {
        self.delete_region(region_document.clone().into());
        self.add_region(region_document.clone().into());
    }
}
