use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use bevy_persistent::{Persistent, StorageFormat};
use brg_core::prelude::consts::path_config_dir;

use crate::features::res::ResEditorFeaturesState;
use crate::features::sys::{display_editor_features_window, toggle_features_window};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(Persistent::<ResEditorFeaturesState>::builder()
                .name("editor features")
                .format(StorageFormat::Json)
                .path(path_config_dir().join("editor-features.json"))
                .default(ResEditorFeaturesState::default())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to insert `ResEditorFeaturesState` due to persistent config error")
            )
            .add_systems(Update, toggle_features_window)
            .add_systems(Update, display_editor_features_window)
        //-
        ;
    }
}
