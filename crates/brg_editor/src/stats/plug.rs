use bevy::app::App;
use bevy::prelude::{apply_deferred, IntoSystemConfigs, Plugin, Update};
use iyes_perf_ui::{PerfUiPlugin, PerfUiSet};

use super::enums::EProfUIState;
use super::res::ResEditorStats;
use super::sys::{despawn_ui_on_state_changed, spawn_ui_if_required, switch_ui_on_keyboard};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResEditorStats {
                current: EProfUIState::Nothing,
                next:    EProfUIState::Mini,
                ui:      None,
            })
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(
                Update,
                (
                    switch_ui_on_keyboard,
                    despawn_ui_on_state_changed,
                    apply_deferred,
                    spawn_ui_if_required,
                    apply_deferred,
                )
                    .before(PerfUiSet::Setup),
            );
    }
}
