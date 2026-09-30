use crate::types::foundry::grid::{JsGridOffset2D, JsGridOffset3D};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GridOffset2D {
    pub i: i32,
    pub j: i32,
}

impl PartialOrd for GridOffset2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.j.partial_cmp(&other.j) {
            Some(core::cmp::Ordering::Equal) => self.i.partial_cmp(&other.i),
            ord => ord,
        }
    }
}

impl Ord for GridOffset2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.j.cmp(&other.j) {
            std::cmp::Ordering::Equal => self.i.cmp(&other.i),
            ord => ord,
        }
    }
}

impl From<GridOffset3D> for GridOffset2D {
    fn from(GridOffset3D { i, j, k: _ }: GridOffset3D) -> Self {
        GridOffset2D { i, j }
    }
}

impl From<JsGridOffset2D> for GridOffset2D {
    fn from(value: JsGridOffset2D) -> Self {
        GridOffset2D { i: value.i() as i32, j: value.j() as i32 }
    }
}

impl From<JsGridOffset3D> for GridOffset2D {
    fn from(value: JsGridOffset3D) -> Self {
        GridOffset2D { i: value.i() as i32, j: value.j() as i32 }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GridOffset3D {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl PartialOrd for GridOffset3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.j.partial_cmp(&other.j) {
            Some(core::cmp::Ordering::Equal) => match self.i.partial_cmp(&other.i) {
                Some(core::cmp::Ordering::Equal) => self.k.partial_cmp(&other.k),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl Ord for GridOffset3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.j.cmp(&other.j) {
            core::cmp::Ordering::Equal => match self.i.cmp(&other.i) {
                core::cmp::Ordering::Equal => self.k.cmp(&other.k),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl From<GridOffset2D> for GridOffset3D {
    fn from(GridOffset2D { i, j }: GridOffset2D) -> Self {
        GridOffset3D { i, j, k: 0 }
    }
}

impl From<JsGridOffset2D> for GridOffset3D {
    fn from(value: JsGridOffset2D) -> Self {
        GridOffset3D { i: value.i() as i32, j: value.j() as i32, k: 0 }
    }
}

impl From<JsGridOffset3D> for GridOffset3D {
    fn from(value: JsGridOffset3D) -> Self {
        GridOffset3D { i: value.i() as i32, j: value.j() as i32, k: value.k() as i32 }
    }
}
