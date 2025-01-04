use bevy::prelude::{Component, Reflect};

#[derive(PartialEq, Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, Reflect)]
pub enum ETeam {
    #[default]
    Neutral,
    Player,
    Enemies,
}

#[derive(Component, Default, Debug, Reflect)]
pub struct CmpTeam {
    pub team: ETeam,
}

impl CmpTeam {
    pub fn new(team: ETeam) -> Self {
        Self { team }
    }
}

impl ETeam {
    pub fn is_friendly_with(self, other: ETeam) -> bool {
        !self.is_enemy_with(other)
    }

    pub fn is_enemy_with(self, other: ETeam) -> bool {
        if self == ETeam::Neutral {
            return false;
        }

        if other == ETeam::Neutral {
            return false;
        }

        if self == other {
            return false;
        }

        true
    }
}
