use bevy::prelude::{Component, Reflect};

#[derive(Default, Debug, Reflect, Copy, Clone, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemies,
    #[default]
    Neutral,
}

#[derive(Component, Default, Debug, Reflect)]
pub struct CmpTeam {
    pub team: Team,
}

impl Team {
    pub fn is_friendly_with(self, other: Team) -> bool {
        !self.is_enemy_with(other)
    }

    pub fn is_enemy_with(self, other: Team) -> bool {
        if self == Team::Neutral {
            return false;
        }

        if other == Team::Neutral {
            return false;
        }

        if self == other {
            return false;
        }

        return true;
    }
}
