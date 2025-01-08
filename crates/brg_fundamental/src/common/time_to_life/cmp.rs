use std::time::Duration;

use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect)]
pub struct CmpTimeToLife {
    pub left: Duration,
}
