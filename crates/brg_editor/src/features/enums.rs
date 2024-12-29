use strum::{Display, EnumIter};

#[derive(EnumIter, Display)]
pub enum EditorFeature {
    ShowWorldOriginAxis,
}
