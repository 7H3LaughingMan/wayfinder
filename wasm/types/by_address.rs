use geo::{BoundingRect, Contains, Coord, Intersects, Rect};
use rstar::{AABB, RTreeObject};
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Display},
    hash::Hash,
    ptr,
    rc::Rc,
};

#[derive(Clone, Default)]
pub struct ByAddress<T>(Rc<RefCell<T>>);

impl<T> ByAddress<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    fn addr(&self) -> *const RefCell<T> {
        Rc::as_ptr(&self.0)
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

impl<T> Debug for ByAddress<T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

impl<T> Display for ByAddress<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

impl<T> PartialEq for ByAddress<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.addr(), other.addr())
    }
}

impl<T> Eq for ByAddress<T> {}

impl<T> Ord for ByAddress<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.addr().cmp(&other.addr())
    }
}

impl<T> PartialOrd for ByAddress<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.addr().partial_cmp(&other.addr())
    }
}

impl<T> Hash for ByAddress<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.addr().hash(state);
    }
}

impl<T> RTreeObject for ByAddress<T>
where T: BoundingRect<f64, Output = Rect<f64>>
{
    type Envelope = AABB<Coord>;

    fn envelope(&self) -> Self::Envelope {
        let bounds = self.borrow().bounding_rect();
        Self::Envelope::from_corners(bounds.min(), bounds.max())
    }
}

impl<T, Rhs> Contains<Rhs> for ByAddress<T>
where T: Contains<Rhs>
{
    fn contains(&self, rhs: &Rhs) -> bool {
        self.borrow().contains(rhs)
    }
}

impl<T, Rhs> Intersects<Rhs> for ByAddress<T>
where T: Intersects<Rhs>
{
    fn intersects(&self, rhs: &Rhs) -> bool {
        self.borrow().intersects(rhs)
    }
}
