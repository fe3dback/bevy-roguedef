use strum::{EnumCount, EnumIter};

#[derive(PartialEq, Eq, Hash, EnumCount, EnumIter, Copy, Clone)]
pub enum EChunkLodLevel {
    /// has full details and vertice for every tile in chunk
    LOD0,

    /// has only 4 vertices for (TL, TR, BL, BR)
    LOD1,
}
