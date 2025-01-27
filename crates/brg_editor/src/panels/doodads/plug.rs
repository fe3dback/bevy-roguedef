use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, OnEnter, Plugin, Update};
use brg_scene::prelude::{GameSystemSet, Loaded};

use super::res_state::ResPanelState;
use super::sys_display_panel::sys_display_panel;
use super::sys_init_state::sys_init_state;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResPanelState::default())
            // 
            .add_systems(OnEnter(Loaded), sys_init_state.in_set(GameSystemSet::ALLOW_ON_LOAD__LoadingSystem))
            .add_systems(Update, sys_display_panel.in_set(GameSystemSet::EDITOR_ONLY__DrawEditorEguiPanels))
        //-
        ;
    }
}
