use crate::prelude::types::Meter;
use crate::prelude::V2;

pub const T_LIB_TILE_SIZE: Meter = 1.0;
pub const T_LIB_TILE_SIZE_SQ: V2 = V2::splat(T_LIB_TILE_SIZE);

/// How many elements(tiles) contains in one parent(chunk) (in one row)
pub const T_LIB_CONT_WIDTH: i32 = 15;
pub const T_LIB_CONT_HEIGHT: i32 = 15;
pub const T_LIB_CONT_WIDTH_HALF: f32 = T_LIB_CONT_WIDTH as f32 / 2.0;

/// One container contains 15x15 elements = 225
pub const T_LIB_CONT_SIZE_SQ: i32 = T_LIB_CONT_WIDTH * T_LIB_CONT_WIDTH;

/// How many tiles contains around chunk center
/// in every direction (ceil(15/2)-1) = 7
pub const T_LIB_ELEMENTS_AROUND_CENTER: i32 = 7;

// based on SQ size of 15, this is indexes for corner and center elements

/// Top left element = 0
pub const T_LIB_ELEM_INDEX_TL: usize = 0;

/// Top right element = 14
pub const T_LIB_ELEM_INDEX_TR: usize = T_LIB_CONT_WIDTH as usize - 1;

/// bottom left element = 210
pub const T_LIB_ELEM_INDEX_BL: usize = T_LIB_CONT_SIZE_SQ as usize - T_LIB_CONT_WIDTH as usize;

/// bottom right element = 224
pub const T_LIB_ELEM_INDEX_BR: usize = T_LIB_CONT_SIZE_SQ as usize - 1;

/// center element = 112
pub const T_LIB_ELEM_INDEX_CENTER: usize = T_LIB_ELEM_INDEX_BR / 2;
