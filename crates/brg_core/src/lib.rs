use bevy::app::{App, Plugin};

mod consts;
mod consts_types;
mod coord;
mod gizmos;
pub mod heightmap;
mod tiles;
mod transform;
mod vectors;

pub mod prelude {
    pub use crate::coord::prelude::*;
    pub use crate::gizmos::prelude::*;
    pub use crate::heightmap::prelude::*;
    pub use crate::tiles::prelude::*;
    pub use crate::transform::prelude::*;
    pub use crate::vectors::prelude::*;

    pub mod types {
        pub use crate::consts_types::prelude::*;
    }

    pub mod consts {
        pub use crate::consts::prelude::*;
    }
}

pub struct BrgCorePlugin;

impl Plugin for BrgCorePlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_plugins(transform::plug::Plug)
        .add_plugins(coord::plug::Plug)
        .add_plugins(heightmap::plug::Plug)
        //-
        ;
    }
}
