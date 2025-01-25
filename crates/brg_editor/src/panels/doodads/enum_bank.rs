use strum::{Display, EnumIter};

#[derive(Default, EnumIter, Display, PartialEq, Eq, Copy, Clone)]
pub enum EBank {
    #[default]
    Village,
}
