use crate::prelude::types::Meter;
use crate::prelude::V2;

pub const T_LIB_TILE_SIZE: Meter = 1.0;
pub const T_LIB_TILE_SIZE_SQ: V2 = V2::splat(T_LIB_TILE_SIZE);

/// How many elements(tiles) contains in one parent(chunk) (in one row)
pub const T_LIB_CONT_ROW_LEN: usize = 16;
pub const T_LIB_CONT_ROW_LEN_HALF: usize = T_LIB_CONT_ROW_LEN / 2;

/// One container contains 16x16 elements = 256
pub const T_LIB_CONT_SIZE_SQ: usize = T_LIB_CONT_ROW_LEN * T_LIB_CONT_ROW_LEN;

// based on SQ size of 16, this is indexes for corner and center elements

/// Top left element = 0
pub const T_LIB_ELEM_INDEX_TL: usize = 0;

/// Top right element = 15
pub const T_LIB_ELEM_INDEX_TR: usize = T_LIB_CONT_ROW_LEN - 1;

/// bottom left element = 240
pub const T_LIB_ELEM_INDEX_BL: usize = T_LIB_CONT_SIZE_SQ - T_LIB_CONT_ROW_LEN;

/// bottom right element = 255
pub const T_LIB_ELEM_INDEX_BR: usize = T_LIB_CONT_SIZE_SQ - 1;

/// center element = 127
pub const T_LIB_ELEM_INDEX_CENTER: usize = 127;
