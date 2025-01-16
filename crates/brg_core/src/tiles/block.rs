use std::fmt::{Debug, Display, Formatter};

use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

pub trait Block {
    fn at(x: i32, y: i32) -> Self;
}

pub trait BlockXY {
    fn x(&self) -> i32;
    fn set_x(&mut self, x: i32);
    fn y(&self) -> i32;
    fn set_y(&mut self, y: i32);
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Reflect, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Reflect, Hash, Serialize, Deserialize)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Reflect, Hash, Serialize, Deserialize)]
pub struct Area {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Reflect, Hash, Serialize, Deserialize)]
pub struct Cluster {
    pub x: i32,
    pub y: i32,
}

macro_rules! impl_block {
    ($blockStruct:ident) => {
        impl Block for $blockStruct {
            #[inline(always)]
            fn at(x: i32, y: i32) -> $blockStruct {
                $blockStruct { x, y }
            }
        }

        impl Display for $blockStruct {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({},{})", stringify!($blockStruct), self.x, self.y)
            }
        }

        impl Debug for $blockStruct {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({},{})", stringify!($blockStruct), self.x, self.y)
            }
        }

        impl BlockXY for $blockStruct {
            #[inline(always)]
            fn x(&self) -> i32 {
                self.x
            }

            #[inline(always)]
            fn set_x(&mut self, x: i32) {
                self.x = x;
            }

            #[inline(always)]
            fn y(&self) -> i32 {
                self.y
            }

            #[inline(always)]
            fn set_y(&mut self, y: i32) {
                self.y = y;
            }
        }
    };
}

impl_block!(Tile);
impl_block!(Chunk);
impl_block!(Area);
impl_block!(Cluster);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_xy() {
        assert_eq!(Tile::at(1, 2), Tile { x: 1, y: 2 });
        assert_eq!(Cluster::at(-21, 1293), Cluster { x: -21, y: 1293 });
    }
}
