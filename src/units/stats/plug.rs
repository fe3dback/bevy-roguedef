use bevy::app::{App, Plugin};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .add_plugins(super::health::plug::Plug)
        //-
        ;
    }
}
