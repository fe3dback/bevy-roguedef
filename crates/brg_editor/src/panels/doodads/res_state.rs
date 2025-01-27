use bevy::prelude::Resource;
use bevy::utils::HashMap;
use brg_core::prelude::{ICDoodads, ICDoodadsCategory, IdOf};

#[derive(Resource, Default)]
pub struct ResPanelState {
    pub initialized:       bool,
    pub available_assets:  HashMap<IdOf<ICDoodadsCategory>, Vec<IdOf<ICDoodads>>>,
    pub selected_category: Option<IdOf<ICDoodadsCategory>>,
}
