use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, ResMut};
use brg_fundamental::prelude::SupHeightmap;

use super::res_state::ResLandscapeState;

#[derive(SystemParam)]
pub struct SupLandscape<'w, 's> {
    pub cmd: Commands<'w, 's>,

    pub hm:    SupHeightmap<'w>,
    pub state: ResMut<'w, ResLandscapeState>,
}
