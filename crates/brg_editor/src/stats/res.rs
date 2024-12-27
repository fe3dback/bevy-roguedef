use bevy::prelude::{Entity, Resource};

use super::enums::EProfUIState;

#[derive(Resource, Default)]
pub struct ResEditorStats {
    pub current: EProfUIState,
    pub next:    EProfUIState,
    pub ui:      Option<Entity>,
}
