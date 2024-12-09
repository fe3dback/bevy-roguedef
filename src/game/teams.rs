use bevy::prelude::{Component, Reflect};

#[derive(Default, Debug, Reflect, Copy, Clone)]
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
