mod features;
mod inspector;
pub mod stats;

use bevy::app::{App, Plugin};

pub mod prelude {
    pub use super::features::prelude::*;
}

pub struct BrgEditorPlugin;

impl Plugin for BrgEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(inspector::plug::Plug)
            .add_plugins(stats::plug::Plug)
            .add_plugins(features::plug::Plug)
        //- 
        ;
    }
}
