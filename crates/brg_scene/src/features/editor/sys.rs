use bevy::prelude::{warn, ButtonInput, KeyCode, Res, ResMut, With, World};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_persistent::Persistent;
use strum::IntoEnumIterator;

use super::res::ResEditorFeaturesState;
use crate::prelude::EditorFeature;

pub fn toggle_features_window(
    mut data: ResMut<Persistent<ResEditorFeaturesState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        if let Err(e) = data.update(|data| data.enabled = !data.enabled) {
            warn!("editor editor window: activate: {}", e);
        }
    }
}

pub fn display_editor_features_window(world: &mut World) {
    world.resource_scope::<Persistent<ResEditorFeaturesState>, _>(|world, mut data| {
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

        egui::Window::new("Editor editor").show(ctx.get_mut(), |ui| {
            for feature in EditorFeature::iter() {
                let id = feature.to_string();
                if data.features.get(&id).is_none() {
                    data.features.insert(id.clone(), false);
                }

                ui.horizontal(|row| {
                    if row
                        .checkbox(
                            &mut data.features.get_mut(&id).unwrap(),
                            feature.to_string(),
                        )
                        .changed()
                    {
                        _ = data.persist();
                    }
                });
            }
        });
    });
}
