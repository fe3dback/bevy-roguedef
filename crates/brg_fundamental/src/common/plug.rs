use bevy::app::{App, Plugin};

use crate::common::time_to_life;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .add_plugins(time_to_life::plug::Plug)
        //-
        ;
    }
}
