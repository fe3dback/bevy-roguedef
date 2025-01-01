use bevy::app::{App, Plugin};

mod consts;
mod consts_types;
mod tiles;
mod vectors;

pub mod prelude {
    pub use crate::tiles::prelude::*;
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
        //-
        ;
    }
}
