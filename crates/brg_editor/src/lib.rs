mod inspector;
pub mod stats;

use bevy::app::{App, Plugin};

pub struct BrgEditorPlugin;

impl Plugin for BrgEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(inspector::plug::Plug)
            .add_plugins(stats::plug::Plug)
        //- 
        ;
    }
}
