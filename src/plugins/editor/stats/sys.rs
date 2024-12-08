use {
    crate::plugins::editor::stats::{enums::EProfUIState, res::ResEditorStats},
    bevy::prelude::{default, ButtonInput, Commands, DespawnRecursiveExt, KeyCode, Res, ResMut},
    iyes_perf_ui::prelude::{PerfUiAllEntries, PerfUiEntryFPS, PerfUiEntryFPSWorst, PerfUiRoot},
};

pub fn switch_ui_on_keyboard(kbr: Res<ButtonInput<KeyCode>>, mut state: ResMut<ResEditorStats>) {
    if kbr.just_pressed(KeyCode::F12) {
        state.next = match state.current {
            EProfUIState::Nothing => EProfUIState::Mini,
            EProfUIState::Mini => EProfUIState::Full,
            EProfUIState::Full => EProfUIState::Nothing,
        };
    }
}

pub fn despawn_ui_on_state_changed(mut cmd: Commands, mut stats: ResMut<ResEditorStats>) {
    if stats.current != stats.next {
        match stats.ui {
            None => {}
            Some(prev_ui) => {
                cmd.entity(prev_ui).despawn_recursive();
            }
        }

        stats.current = EProfUIState::Nothing;
        stats.ui = None;
    }
}

pub fn spawn_ui_if_required(mut cmd: Commands, mut stats: ResMut<ResEditorStats>) {
    if stats.current == stats.next {
        return;
    }

    stats.current = stats.next;
    stats.ui = match stats.current {
        EProfUIState::Nothing => None,
        EProfUIState::Mini => Some(
            cmd.spawn((
                PerfUiRoot {
                    display_labels: false,
                    layout_horizontal: true,
                    ..default()
                },
                PerfUiEntryFPSWorst::default(),
                PerfUiEntryFPS::default(),
            ))
            .id(),
        ),
        EProfUIState::Full => Some(cmd.spawn(PerfUiAllEntries::default()).id()),
    }
}
