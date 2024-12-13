const CONTAINER_LEN: i32 = 15;
// 15 * 15 = 225
const AROUND_CENTER: i32 = 7; // CONTAINER_LEN/2

// How many tiles contains in one chunk (in one row)
// One chunk contains 15x15 tiles = 225 tiles
pub const TILES_PER_CHUNK: i32 = CONTAINER_LEN;
pub const TILES_PER_CHUNK_HALF: f32 = TILES_PER_CHUNK as f32 / 2.0;

// How many tiles contains around chunk center
// in every direction (ceil(15/2)-1) = 7
pub const TILES_AROUND_CENTER: i32 = AROUND_CENTER;

// How many chunks contains in one area (in one row)
// One area contains 15x15 chunks = 225 chunks
pub const CHUNKS_PER_AREA: i32 = CONTAINER_LEN;
pub const CHUNKS_PER_AREA_HALF: f32 = CHUNKS_PER_AREA as f32 / 2.0;

// How many chunks contains around area center
// in every direction (ceil(15/2)-1) = 7
// pub const CHUNKS_AROUND_CENTER: i32 = 7; // todo: required?
