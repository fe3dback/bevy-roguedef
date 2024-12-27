use bevy::app::{App, Plugin};

mod consts;
mod consts_types;
mod gizmos;
mod tiles;
mod transform;
mod vectors;

pub mod prelude {
    pub use crate::gizmos::prelude::*;
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
        //-
        ;
    }
}
