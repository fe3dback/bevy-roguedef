use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Copy, Clone)]
pub enum EditorFeature {
    GizmosOriginAxis,
    GizmosGridTiles,
    GizmosWorldMouse,
    VolumesCollision,
    VolumesProjectile,
    LandscapeHeightmap,
}
