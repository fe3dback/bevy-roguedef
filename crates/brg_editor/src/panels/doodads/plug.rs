use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use brg_scene::prelude::GameSystemSet;

use super::res_state::ResPanelState;
use super::sys_display_panel::sys_display_panel;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResPanelState::default())
            // 
            .add_systems(Update, sys_display_panel.in_set(GameSystemSet::EDITOR_ONLY__DrawEditorEguiPanels))
        //-
        ;
    }
}
