use bevy::app::{App, Plugin};
use brg_core::prelude::ResRandomSource;

use super::enum_randomizer_kind::{
    RandomizerKindSounds,
    RandomizerKindSpawn,
    RandomizerKindSpells,
};
use super::time_to_life;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .add_plugins(time_to_life::plug::Plug)
            .insert_resource(ResRandomSource::<RandomizerKindSpawn>::new())
            .insert_resource(ResRandomSource::<RandomizerKindSounds>::new())
            .insert_resource(ResRandomSource::<RandomizerKindSpells>::new())
        //-
        ;
    }
}
