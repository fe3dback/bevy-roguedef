use std::time::Duration;

use bevy::prelude::{Component, Entity, Reflect};
use brg_core::prelude::V2;

#[derive(Component)]
pub struct CmpAiControllable;

#[derive(Component, Reflect)]
pub struct CmpAiBehaviorSimple {
    pub agro_distance:        f32,
    pub since_last_scan:      Duration,
    pub last_known_enemy_pos: V2,
    pub last_target:          Option<Entity>,
}

impl Default for CmpAiBehaviorSimple {
    fn default() -> Self {
        Self {
            agro_distance:        10.0,
            since_last_scan:      Duration::default(),
            last_known_enemy_pos: V2::default(),
            last_target:          None,
        }
    }
}
