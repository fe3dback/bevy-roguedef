use brg_core::prelude::V2;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct MeshIdent {
    pub pos:   V2,
    pub size:  V2,
    pub depth: u8,
}
