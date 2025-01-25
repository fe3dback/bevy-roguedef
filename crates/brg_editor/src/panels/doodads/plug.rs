use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use brg_scene::prelude::GameSystemSet;

use super::sys_display_panel::sys_display_panel;
use super::res_state::ResPanelState;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResPanelState::default())
            // 
            .add_systems(Update, sys_display_panel.in_set(GameSystemSet::Editor_Draw_Panels))
        //-
        ;
    }
}
