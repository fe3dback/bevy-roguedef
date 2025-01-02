use strum::{Display, EnumIter};

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy, Hash, EnumIter)]
pub enum MobKind {
    #[strum(to_string = "player")]
    Player,
    #[strum(to_string = "slime_small")]
    SlimeSmall,
    #[strum(to_string = "slime_big")]
    SlimeBig,
}
