use bevy::prelude::{Component, Reflect};
use bevy_inspector_egui::InspectorOptions;

use crate::components::lib::V2;
use crate::components::tiles::{Range, Tile};
use crate::components::unit_creature::CmpUnitBuilding;
use crate::game::buildings::electro::dto::ChannelState;
use crate::game::buildings::electro::types::MAX_CHANNELS;
use crate::game::energy::CmpEnergyContainer;

#[derive(Component, Reflect, InspectorOptions, Debug, Default, Clone)]
#[require(CmpUnitBuilding)]
pub struct CmpBuildingElectricity {
    pub is_source:                  bool,
    pub can_consume:                bool,
    pub can_produce:                bool,
    pub capacity:                   f32,
    pub count_channels:             u8,
    pub channels:                   [ChannelState; MAX_CHANNELS],
    pub throughput_max_in:          f32,
    pub throughput_max_out:         f32,
    pub connection_radius_in_tiles: f32,
}

impl CmpEnergyContainer for CmpBuildingElectricity {
    fn try_spend(&mut self, amount: f32) -> bool {
        let mut dirty = self.channels;

        let mut reminder = amount;
        for chan in &mut dirty {
            let spend = f32::min(reminder, chan.charge);
            chan.charge -= spend;
            reminder -= spend;
        }

        if reminder > 0.0 {
            return false;
        }

        self.channels = dirty;
        true
    }
}

#[derive(Component, Reflect, InspectorOptions, Default)]
pub struct CmpBuildingOccupied {
    pub grid_position: Tile,
    pub grid_width:    u8,
    pub grid_height:   u8,
}

impl CmpBuildingOccupied {
    #[inline(always)]
    pub fn range(&self) -> Range {
        Range::new(
            self.grid_position.x,
            self.grid_position.y,
            self.grid_position.x + (self.grid_width - 1) as i32,
            self.grid_position.y + (self.grid_height - 1) as i32,
        )
    }

    #[inline(always)]
    pub fn center(&self) -> V2 {
        self.range().position_center()
    }
}
