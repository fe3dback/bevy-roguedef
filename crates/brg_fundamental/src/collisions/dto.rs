use brg_core::prelude::V2;

use crate::prelude::CmpCollisionVolume;

#[derive(Copy, Clone)]
pub struct DtoCollisionMovingObject {
    pub pos_current: V2,
    pub pos_desired: V2,
    pub volume:      CmpCollisionVolume,
}

#[derive(Copy, Clone)]
pub struct DtoCollisionStaticObject {
    pub pos:    V2,
    pub volume: CmpCollisionVolume,
}

pub struct DtoCollisionHit {
    pub pos: V2,
}
