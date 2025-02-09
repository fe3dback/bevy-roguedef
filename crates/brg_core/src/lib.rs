use bevy::app::{App, Plugin};

mod consts;
mod consts_types;
mod debug;
mod files;
mod id;
mod rand;
mod tiles;
mod vectors;

pub mod prelude {
    pub use crate::debug::prelude::*;
    pub use crate::files::prelude::*;
    pub use crate::id::prelude::*;
    pub use crate::rand::prelude::*;
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
            .add_plugins(rand::plug::Plug)
        //-
        ;
    }
}
