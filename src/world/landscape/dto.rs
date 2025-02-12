use brg_core::prelude::V2;

use super::sup_mesh::NeighbourSizeTransition;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct MeshIdent {
    pub pos:        V2,
    pub size:       V2,
    pub depth:      u8,
    pub transition: NeighbourSizeTransition,
}
