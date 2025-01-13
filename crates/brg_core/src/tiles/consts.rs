use crate::prelude::types::Meter;
use crate::prelude::V2;

pub const TILE_SIZE: Meter = 1.0;
pub const TILE_SIZE_SQ: V2 = V2::splat(TILE_SIZE);

// How many elements(tiles) contains in one parent(chunk) (in one row)
// One chunk contains 15x15 tiles = 225 tiles
pub const ELEMENTS_IN_CONTAINER: i32 = 15;
pub const ELEMENTS_IN_CONTAINER_HALF: f32 = ELEMENTS_IN_CONTAINER as f32 / 2.0;

// How many tiles contains around chunk center
// in every direction (ceil(15/2)-1) = 7
pub const ELEMENTS_AROUND_CENTER: i32 = 7;
