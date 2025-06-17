use crate::types::Rectangle;
use by_address::ByAddress;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub type QuadPointer<T> = Rc<RefCell<T>>;
pub type CollisionCheck<T> = fn(&QuadPointer<QuadtreeObject<T>>, &Rectangle) -> bool;

#[derive(Debug)]
pub struct QuadtreeObject<T> {
    pub r: Rectangle,
    pub t: QuadPointer<T>,
    pub n: HashSet<ByAddress<QuadPointer<Quadtree<T>>>>,
}

impl<T> QuadtreeObject<T> {
    pub fn new(r: Rectangle, t: QuadPointer<T>) -> QuadPointer<Self> {
        Rc::new(RefCell::new(Self { r, t, n: HashSet::new() }))
    }
}

#[derive(Debug)]
pub struct Quadtree<T> {
    pub bounds: Rectangle,
    pub objects: Vec<QuadPointer<QuadtreeObject<T>>>,
    pub nodes: Vec<QuadPointer<Quadtree<T>>>,

    pub _max_objects: usize,
    pub _max_depth: usize,
    pub _depth: usize,
    pub _root: Option<ByAddress<QuadPointer<Quadtree<T>>>>,
    pub _this: Option<ByAddress<QuadPointer<Quadtree<T>>>>,
}

impl<T> Quadtree<T> {
    pub fn new(
        bounds: Rectangle,
        depth: Option<usize>,
        max_depth: Option<usize>,
        max_objects: Option<usize>,
        root: Option<ByAddress<QuadPointer<Quadtree<T>>>>,
    ) -> QuadPointer<Self> {
        let pointer = Rc::new(RefCell::new(Self {
            bounds,
            objects: Vec::new(),
            nodes: Vec::new(),
            _max_objects: max_objects.unwrap_or(20),
            _max_depth: max_depth.unwrap_or(4),
            _depth: depth.unwrap_or(0),
            _root: root.clone(),
            _this: None,
        }));

        let mut this = pointer.borrow_mut();
        this._this = Some(ByAddress(pointer.clone()));

        if root.is_none() {
            this._root = Some(ByAddress(pointer.clone()));
        }
        return pointer.clone();
    }

    pub fn all(&self) -> Vec<QuadPointer<QuadtreeObject<T>>> {
        if self.nodes.len() != 0 {
            return self.nodes.iter().flat_map(|n| n.as_ref().borrow().all()).collect();
        }

        return self.objects.clone();
    }

    pub fn split(&mut self) -> QuadPointer<Quadtree<T>> {
        let b = self.bounds;
        let w = b.width / 2.0;
        let h = b.height / 2.0;

        self.nodes = vec![
            Quadtree::new(
                Rectangle::new(b.x, b.y, w, h),
                Some(self._depth + 1),
                Some(self._max_depth),
                Some(self._max_objects),
                self._this.clone(),
            ),
            Quadtree::new(
                Rectangle::new(b.x + w, b.y, w, h),
                Some(self._depth + 1),
                Some(self._max_depth),
                Some(self._max_objects),
                self._this.clone(),
            ),
            Quadtree::new(
                Rectangle::new(b.x, b.y + h, w, h),
                Some(self._depth + 1),
                Some(self._max_depth),
                Some(self._max_objects),
                self._this.clone(),
            ),
            Quadtree::new(
                Rectangle::new(b.x + w, b.y + h, w, h),
                Some(self._depth + 1),
                Some(self._max_depth),
                Some(self._max_objects),
                self._this.clone(),
            ),
        ];

        let objects: Vec<QuadPointer<QuadtreeObject<T>>> = self.objects.iter().map(|o| o.clone()).collect();

        for o in objects {
            o.as_ref().borrow_mut().n.remove(&self._this.clone().unwrap());
            self.insert(o.clone());
        }
        return self._this.clone().unwrap().0;
    }

    pub fn clear(&mut self) -> QuadPointer<Quadtree<T>> {
        self.objects.clear();
        for node in &self.nodes {
            node.borrow_mut().clear();
        }
        self.nodes.clear();
        return self._this.clone().unwrap().0;
    }

    pub fn insert(&mut self, object: QuadPointer<QuadtreeObject<T>>) -> Vec<QuadPointer<Quadtree<T>>> {
        if self.objects.len() == (self._max_objects - 1) && self._depth < self._max_depth {
            if self.nodes.len() == 0 {
                self.split();
            }
        }

        if self.nodes.len() != 0 {
            let nodes = self.get_child_nodes(&object.borrow().r);
            return nodes.iter().flat_map(|n| n.borrow_mut().insert(object.clone())).collect();
        }

        object.borrow_mut().n.insert(self._this.clone().unwrap());
        self.objects.push(object);
        return vec![self._this.clone().unwrap().0];
    }

    pub fn remove(&mut self, target: QuadPointer<T>) -> QuadPointer<Quadtree<T>> {
        let target_address = ByAddress(target.clone());
        self.objects.retain(|o| ByAddress(o.as_ref().borrow().t.clone()) != target_address);
        for n in &self.nodes {
            n.borrow_mut().remove(target.clone());
        }
        return self._this.clone().unwrap().0;
    }

    pub fn update(&mut self, object: QuadPointer<QuadtreeObject<T>>) -> Vec<QuadPointer<Quadtree<T>>> {
        self.remove((*object).borrow().t.clone());
        return self.insert(object);
    }

    pub fn get_objects(&self, rect: Rectangle) -> Vec<QuadPointer<T>> {
        let objects: QuadPointer<HashSet<ByAddress<QuadPointer<T>>>> = Rc::new(RefCell::new(HashSet::new()));

        self._get_objects(&rect, &None, &objects);

        return (*objects).borrow().iter().map(|o| o.0.clone()).collect();
    }

    pub fn _get_objects(
        &self,
        rect: &Rectangle,
        collision_test: &Option<CollisionCheck<T>>,
        set: &QuadPointer<HashSet<ByAddress<QuadPointer<T>>>>,
    ) {
        if self.nodes.len() != 0 {
            let nodes = self.get_child_nodes(rect);
            for node in &nodes {
                node.borrow()._get_objects(rect, collision_test, set);
            }
        } else {
            match collision_test {
                Some(f) => self.objects.iter().for_each(|o| {
                    if rect.overlaps(&o.borrow().r) && f(o, rect) {
                        set.borrow_mut().insert(ByAddress(o.borrow().t.clone()));
                    }
                }),
                None => self.objects.iter().for_each(|o| {
                    if rect.overlaps(&o.borrow().r) {
                        set.borrow_mut().insert(ByAddress(o.borrow().t.clone()));
                    }
                }),
            }
        }
    }

    pub fn get_leaf_nodes(&self, rect: &Rectangle) -> Vec<QuadPointer<Quadtree<T>>> {
        if self.nodes.len() == 0 {
            return vec![self._this.clone().unwrap().0];
        }

        let nodes = self.get_child_nodes(rect);
        return nodes.iter().flat_map(|n| n.as_ref().borrow().get_leaf_nodes(rect)).collect();
    }

    pub fn get_child_nodes(&self, rect: &Rectangle) -> Vec<QuadPointer<Quadtree<T>>> {
        if self.nodes.len() == 0 {
            return vec![self._this.clone().unwrap().0];
        }

        let mut nodes = Vec::new();
        let hx = self.bounds.x + (self.bounds.width / 2.0);
        let hy = self.bounds.y + (self.bounds.height / 2.0);

        let start_top = rect.y <= hy;
        let start_left = rect.x <= hx;
        let end_bottom = (rect.y + rect.height) > hy;
        let end_right = (rect.x + rect.width) > hx;

        if start_left && start_top {
            nodes.push(self.nodes[0].clone());
        }

        if end_right && start_top {
            nodes.push(self.nodes[1].clone());
        }

        if start_left && end_bottom {
            nodes.push(self.nodes[2].clone());
        }

        if end_right && end_bottom {
            nodes.push(self.nodes[3].clone());
        }

        return nodes;
    }
}
