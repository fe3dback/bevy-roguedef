use bevy::prelude::Resource;

use super::enum_bank::EBank;

#[derive(Resource, Default)]
pub struct ResPanelState {
    pub bank: EBank,
}
