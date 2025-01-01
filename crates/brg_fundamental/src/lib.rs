use bevy::prelude::{App, Plugin};

pub mod camera;
mod coord;
mod gizmos;
mod heightmap;
mod transform;

pub mod prelude {
    pub use crate::camera::prelude::*;
    pub use crate::coord::prelude::*;
    pub use crate::gizmos::prelude::*;
    pub use crate::heightmap::prelude::*;
    pub use crate::transform::prelude::*;
}

pub struct BrgFundamentalPlugin;

impl Plugin for BrgFundamentalPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(transform::plug::Plug)
            .add_plugins(heightmap::plug::Plug)
            .add_plugins(coord::plug::Plug)
        //-
        ;
    }
}
