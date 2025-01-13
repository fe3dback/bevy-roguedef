use super::block::{Area, Block, Chunk, Cluster, Tile};
use super::consts::*;

#[allow(dead_code)]
pub trait BlockParent {
    type Parent: Block;

    fn parent(&self) -> Self::Parent;
}

macro_rules! impl_block_parent {
    ($blockStruct:ty, $parent:ty) => {
        impl BlockParent for $blockStruct {
            type Parent = $parent;

            fn parent(&self) -> Self::Parent {
                Self::Parent::at(
                    (((self.x as f32) + ELEMENTS_IN_CONTAINER_HALF)
                        / (ELEMENTS_IN_CONTAINER as f32))
                        .floor() as i32,
                    (((self.y as f32) + ELEMENTS_IN_CONTAINER_HALF)
                        / (ELEMENTS_IN_CONTAINER as f32))
                        .floor() as i32,
                )
            }
        }
    };
}

impl_block_parent!(Tile, Chunk);
impl_block_parent!(Chunk, Area);
impl_block_parent!(Area, Cluster);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent() {
        assert_eq!(Tile::at(-7, -7).parent(), Chunk::at(0, 0));
        assert_eq!(Tile::at(7, 7).parent(), Chunk::at(0, 0));
        assert_eq!(Tile::at(8, 7).parent(), Chunk::at(1, 0));
        assert_eq!(Tile::at(8, 8).parent(), Chunk::at(1, 1));
        assert_eq!(Chunk::at(-8, 8).parent(), Area::at(-1, 1));
    }
}
