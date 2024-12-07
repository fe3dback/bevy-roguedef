use crate::plugins::editor::stats::enums::EProfUIState;
use bevy::prelude::{Entity, Resource};

#[derive(Resource, Default)]
pub struct ResEditorStats {
    pub current: EProfUIState,
    pub next: EProfUIState,
    pub ui: Option<Entity>,
}