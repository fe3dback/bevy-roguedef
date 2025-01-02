use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect)]
pub struct CmpTimeToLife {
    pub seconds_left: f32,
}
