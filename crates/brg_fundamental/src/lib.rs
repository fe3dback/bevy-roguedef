use bevy::prelude::{App, Plugin};

pub mod camera;
mod collisions;
pub mod common;
mod coord;
mod gizmos;
mod heightmap;
mod rendermode;
pub mod sound;
mod transform;

pub mod prelude {
    pub use crate::camera::prelude::*;
    pub use crate::collisions::prelude::*;
    pub use crate::common::prelude::*;
    pub use crate::coord::prelude::*;
    pub use crate::gizmos::prelude::*;
    pub use crate::heightmap::prelude::*;
    pub use crate::sound::prelude::*;
    pub use crate::transform::prelude::*;
}

pub struct BrgFundamentalPlugin;

impl Plugin for BrgFundamentalPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(rendermode::plug::Plug)
            .add_plugins(transform::plug::Plug)
            .add_plugins(heightmap::plug::Plug)
            .add_plugins(coord::plug::Plug)
            .add_plugins(common::plug::Plug)
            .add_plugins(sound::plug::Plug)
            .add_plugins(collisions::plug::Plug)
        //-
        ;
    }
}
