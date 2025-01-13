use bevy::app::{App, Plugin};

mod assets;
mod features;
mod level;
mod state;
mod system_set;

pub mod prelude {
    pub use crate::assets::prelude::*;
    pub use crate::features::prelude::*;
    pub use crate::level::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::system_set::prelude::*;
}

pub struct BrgScenePlugin;

impl Plugin for BrgScenePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(state::plug::Plug)
            .add_plugins(features::plug::Plug)
            .add_plugins(system_set::plug::Plug)
            .add_plugins(assets::plug::Plug)
        //-
        ;
    }
}
