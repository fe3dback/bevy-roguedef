use std::ops::{Add, AddAssign, Sub, SubAssign};

use brg_core::prelude::{V2, V3};

pub trait PointCoord {
    fn xyh(&self) -> V3;
    fn update(&mut self, value: V3);
}

#[derive(Copy, Clone, Debug)]
pub enum Point<T: PointCoord> {
    /// relative to terrain height
    Rel(T),
    /// absolute world height
    Abs(T),
}

impl PointCoord for V2 {
    fn xyh(&self) -> V3 {
        self.with_height(0.0)
    }

    fn update(&mut self, value: V3) {
        self.x = value.x;
        self.y = value.y;
    }
}

impl PointCoord for V3 {
    fn xyh(&self) -> V3 {
        *self
    }

    fn update(&mut self, value: V3) {
        self.x = value.x;
        self.y = value.y;
        self.h = value.h;
    }
}

#[auto_impl_ops::auto_ops]
impl<T: PointCoord> AddAssign<V3> for Point<T> {
    fn add_assign(&mut self, other: V3) {
        match (self) {
            Self::Rel(s) => s.update(s.xyh() + other),
            Self::Abs(s) => s.update(s.xyh() + other),
        }
    }
}

#[auto_impl_ops::auto_ops]
impl<T: PointCoord> SubAssign<V3> for Point<T> {
    fn sub_assign(&mut self, other: V3) {
        match (self) {
            Self::Rel(s) => s.update(s.xyh() - other),
            Self::Abs(s) => s.update(s.xyh() - other),
        }
    }
}
