use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::GameSystemSet;

use super::res::ResRenderModes;
use super::sys_switch_mode::{sys_on_mode_switched, sys_switch_mode_on_keyboard};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResRenderModes::default())
            .add_systems(Update, (sys_switch_mode_on_keyboard, sys_on_mode_switched).in_set(GameSystemSet::Debug_Input))
        //-
        ;
    }
}
