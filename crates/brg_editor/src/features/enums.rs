use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Copy, Clone)]
pub enum EditorFeature {
    ShowWorldOriginAxis,
    ShowWorldMousePosition,
    DrawHeightmapPoints,
    DrawProjectile,
}
