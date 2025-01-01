use bevy::app::{App, Plugin};
use bevy::prelude::PostUpdate;

use super::cmp::CmpTransform2D;
use super::sys::transform_apply;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpTransform2D>()
            .add_systems(PostUpdate, transform_apply)
        //-
        ;
    }
}
