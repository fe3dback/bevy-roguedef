use bevy::prelude::{ButtonInput, KeyCode, Res, ResMut, With, World};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use strum::IntoEnumIterator;

use super::res::ResEditorFeaturesState;
use crate::features::enums::EditorFeature;

pub fn toggle_features_window(
    mut data: ResMut<ResEditorFeaturesState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        data.enabled = !data.enabled;
    }
}

pub fn display_editor_features_window(world: &mut World) {
    world.resource_scope::<ResEditorFeaturesState, _>(|world, mut data| {
        if !data.enabled {
            return;
        }

        let Ok(egui_context) = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .get_single(world)
        else {
            return;
        };

        let mut ctx = egui_context.clone();

        egui::Window::new("Editor features").show(ctx.get_mut(), |ui| {
            for feature in EditorFeature::iter() {
                let id = feature.to_string();
                if data.features.get(&id).is_none() {
                    data.features.insert(id.clone(), false);
                }

                ui.horizontal(|row| {
                    row.checkbox(
                        &mut data.features.get_mut(&id).unwrap(),
                        feature.to_string(),
                    );
                });
            }
        });
    });
}
