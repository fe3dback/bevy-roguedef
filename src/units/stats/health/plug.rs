use bevy::app::{App, Plugin};

use super::cmp_health::CmpHealth;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpHealth>()
        //-
        ;
    }
}
