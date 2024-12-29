use bevy::app::App;
use bevy::prelude::{Plugin, Update};

use crate::features::res::ResEditorFeaturesState;
use crate::features::sys::{display_editor_features_window, toggle_features_window};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResEditorFeaturesState::default())
            .add_systems(Update, toggle_features_window)
            .add_systems(Update, display_editor_features_window)
        //-
        ;
    }
}
