use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use brg_core::prelude::types::Meter;
use brg_core::prelude::{
    remap,
    Area,
    Block,
    BlockChild,
    Chunk,
    Tile,
    T_LIB_CONT_HEIGHT,
    T_LIB_CONT_SIZE_SQ,
    T_LIB_CONT_WIDTH,
    T_LIB_ELEM_INDEX_BL,
    T_LIB_ELEM_INDEX_BR,
    T_LIB_ELEM_INDEX_CENTER,
    T_LIB_ELEM_INDEX_TL,
    T_LIB_ELEM_INDEX_TR,
};
use brg_scene::prelude::{RawLevelArea, RawLevelChunk, RawLevelChunkHeights, RawLevelData};
use exr::prelude::{FlatSamples, IntegerBounds};

#[derive(Debug)]
pub struct LevelBounds {
    input_exr_width:              u32, // real exr image width
    input_exr_height:             u32, // real exr image height
    input_exr_closest_dst_width:  u32, // clamped image width to the closest square (4097 -> 4096)
    input_exr_closest_dst_height: u32, // clamped image height to the closest square (2050 -> 2048)
    input_min_value:              f32, // min pixel grayscale value (aka height/saturation) of all exr pixels
    input_max_value:              f32, // max pixel grayscale value (aka height/saturation) of all exr pixels

    areas_width:    u32, // how many areas in map by width
    areas_height:   u32, // how many areas in map by height
    chunks_width:   u32, // how many chunks in map by width
    chunks_height:  u32, // how many chunks in map by height
    scale_factor_x: u32, // how many tiles will be mapped to single exr pixel by x
    scale_factor_y: u32, // how many tiles will be mapped to single exr pixel bv y
}

#[derive(Debug)]
pub struct Stats {
    empty:   u32,
    splats:  u32,
    minimal: u32,
    lite:    u32,
    full:    u32,
}

#[derive(Eq, PartialEq)]
pub enum KeyChunkKind {
    None,
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

const MAP_SIZE_M: Meter = 4096.0 * 4.0; // 16 km^2
const MAP_NAME: &str = "example";

fn main() -> Result<()> {
    let input_exr_file: PathBuf = PathBuf::from(
        "/home/neo/code/fe3dback/bevy-roguedef/sources/terrain/example/Rugged Terrain with Rocky Peaks Height Map EXR.exr",
    );

    let out_directory: PathBuf = PathBuf::from(format!(
        "/home/neo/code/fe3dback/bevy-roguedef/assets/maps/{}/",
        MAP_NAME
    ));

    // prepare data
    let (bounds, samples) = load_exr(input_exr_file.as_path()).context("failed load exr")?;
    let settings = calculate_map_settings(bounds, &samples);
    println!("[LOADED] settings: {:?}", settings);

    // build areas
    let mut level = RawLevelData::new(MAP_NAME, settings.chunks_width, settings.chunks_height);
    let mut stats = Stats {
        empty:   0,
        splats:  0,
        minimal: 0,
        lite:    0,
        full:    0,
    };

    for abs_area_y in 0..settings.areas_height as i32 {
        for abs_area_x in 0..settings.areas_width as i32 {
            let area = Area::at(abs_area_x, abs_area_y);
            let mut area_data = RawLevelArea::default();

            for chunk in &area.child_range() {
                if chunk.x < 0 || chunk.y < 0 {
                    continue;
                }

                let mut heights: Vec<f32> = Vec::with_capacity(T_LIB_CONT_SIZE_SQ as usize);

                for tile in &chunk.child_range() {
                    let pixel_index = calculate_pixel_index(&settings, tile);
                    let sample = samples.value_by_flat_index(pixel_index);

                    // normalize value
                    let value = remap(
                        settings.input_min_value,
                        settings.input_max_value,
                        0.0,
                        1.0,
                        sample.to_f32(),
                    );

                    heights.push(value);
                }

                let row = T_LIB_CONT_WIDTH as usize;
                let ind_tl = T_LIB_ELEM_INDEX_TL;
                let ind_tr = T_LIB_ELEM_INDEX_TR;
                let ind_bl = T_LIB_ELEM_INDEX_BL;
                let ind_br = T_LIB_ELEM_INDEX_BR;

                let (w, h) = (settings.chunks_width as i32, settings.chunks_height as i32);
                let vert_center = heights[T_LIB_ELEM_INDEX_CENTER];

                let vert_edges = [
                    heights[ind_tl],
                    heights[ind_tr],
                    heights[ind_bl],
                    heights[ind_br],
                ];

                let vert_support = [
                    heights[ind_tl + 4],
                    heights[ind_tr - 4],
                    heights[ind_tl + (row * 4) + 0],
                    heights[ind_tl + (row * 4) + 4],
                    heights[ind_tr + (row * 4) - 0],
                    heights[ind_tr + (row * 4) - 4],
                    heights[ind_bl - (row * 4) + 0],
                    heights[ind_bl - (row * 4) + 4],
                    heights[ind_br - (row * 4) - 0],
                    heights[ind_br - (row * 4) - 4],
                    heights[ind_bl + 4],
                    heights[ind_br - 4],
                ];

                let chunk_kind = is_area_key_chunk(area, chunk);
                if chunk_kind != KeyChunkKind::None {
                    match chunk_kind {
                        KeyChunkKind::Center => {
                            area_data.heights.0 = heights[T_LIB_ELEM_INDEX_CENTER]
                        }
                        KeyChunkKind::TopLeft => {
                            area_data.heights.1[0] = heights[T_LIB_ELEM_INDEX_TL]
                        }
                        KeyChunkKind::TopRight => {
                            area_data.heights.1[1] = heights[T_LIB_ELEM_INDEX_TR]
                        }
                        KeyChunkKind::BottomLeft => {
                            area_data.heights.1[2] = heights[T_LIB_ELEM_INDEX_BL]
                        }
                        KeyChunkKind::BottomRight => {
                            area_data.heights.1[3] = heights[T_LIB_ELEM_INDEX_BR]
                        }
                        _ => continue,
                    }
                }

                let heights = if chunk_in_center_percent_range(chunk, 15.0, w, h) {
                    stats.full += 1;
                    RawLevelChunkHeights::Full(heights)
                } else if chunk_in_center_percent_range(chunk, 30.0, w, h) {
                    stats.lite += 1;
                    RawLevelChunkHeights::Lite(vert_center, vert_edges, vert_support)
                } else if chunk_in_center_percent_range(chunk, 45.0, w, h) {
                    stats.minimal += 1;
                    RawLevelChunkHeights::Minimal(vert_center, vert_edges)
                } else if chunk_in_center_percent_range(chunk, 60.0, w, h) {
                    stats.minimal += 1;
                    RawLevelChunkHeights::Splat(vert_center)
                } else {
                    stats.empty += 1;
                    RawLevelChunkHeights::Empty
                };

                level.chunk_write(chunk, RawLevelChunk { heights });
            }

            level.area_write(area, area_data);
        }
    }

    println!("[Chunks ready] stats: {:?}", stats);

    // write data to files
    let out_path = out_directory.join("world.landscape.bin");
    fs::create_dir_all(&out_path.parent().unwrap()).context("creating map directories")?;

    let mut file = File::create(out_path).context("failed create area file")?;
    let file_content = level.to_bytes().context("failed serialize level data")?;

    file.write_all(&file_content).context("writing bytes")?;

    Ok(())
}

fn load_exr(exr: &Path) -> Result<(IntegerBounds, FlatSamples)> {
    let image_data = exr::prelude::read_all_data_from_file(exr).context("failed read exr file")?;

    let Some(layer) = image_data.layer_data.first() else {
        bail!("exr not have any layers")
    };
    println!("[LOAD EXR] layer: {:?}", layer);

    let chan = &layer.channel_data.list[0];
    println!("[LOAD EXR] channel 0: {:?}", chan.name);

    let samples = &chan.sample_data.levels_as_slice()[0];
    Ok((
        image_data.attributes.display_window.clone(),
        samples.clone(),
    ))
}

fn calculate_map_settings(in_bounds: IntegerBounds, samples: &FlatSamples) -> LevelBounds {
    let (input_exr_width, input_exr_height) =
        (in_bounds.size.x() as u32, in_bounds.size.y() as u32);

    let (input_exr_closest_dst_width, input_exr_closest_dst_height) =
        found_closest_square_bound(in_bounds);

    let map_width_m = MAP_SIZE_M.floor() as u32;
    let map_height_m = MAP_SIZE_M.floor() as u32;

    let (input_min_value, input_max_value) = find_min_max_height(samples);

    let areas_width = (map_width_m as f32 / Area::size_m().x).floor() as u32;
    let areas_height = (map_height_m as f32 / Area::size_m().y).floor() as u32;

    let chunks_width = areas_width * T_LIB_CONT_WIDTH as u32;
    let chunks_height = areas_height * T_LIB_CONT_HEIGHT as u32;

    LevelBounds {
        input_exr_width,
        input_exr_height,
        input_exr_closest_dst_width,
        input_exr_closest_dst_height,
        input_min_value,
        input_max_value,

        areas_width,
        areas_height,
        chunks_width,
        chunks_height,
        scale_factor_x: map_width_m / input_exr_closest_dst_width,
        scale_factor_y: map_height_m / input_exr_closest_dst_height,
    }
}

fn find_min_max_height(samples: &FlatSamples) -> (f32, f32) {
    let mut height_min: f32 = 1000.0;
    let mut height_max: f32 = -1000.0;

    for height in samples.values_as_f32() {
        if height > height_max {
            height_max = height;
        }
        if height < height_min {
            height_min = height;
        }
    }

    (height_min, height_max)
}

#[inline(always)]
fn found_closest_square_bound(in_bounds: IntegerBounds) -> (u32, u32) {
    // for example if image size is 4097x2048
    // this function should return 4096x2048 (closest LOWER square bounds 2^X)

    // todo: remove hardcoded
    (4096, 4096)
}

fn is_area_key_chunk(area: Area, chunk: Chunk) -> KeyChunkKind {
    if area.child_elem_center() == chunk {
        return KeyChunkKind::Center;
    }

    if area.child_elem_top_left() == chunk {
        return KeyChunkKind::TopLeft;
    }

    if area.child_elem_top_right() == chunk {
        return KeyChunkKind::TopRight;
    }

    if area.child_elem_bottom_left() == chunk {
        return KeyChunkKind::BottomLeft;
    }

    if area.child_elem_bottom_right() == chunk {
        return KeyChunkKind::BottomRight;
    }

    KeyChunkKind::None
}

#[inline(always)]
fn calculate_pixel_index(settings: &LevelBounds, tile: Tile) -> usize {
    let sample_x = (tile.x as f32 / settings.scale_factor_x as f32).floor() as u32;
    let sample_y = (tile.y as f32 / settings.scale_factor_y as f32).floor() as u32;

    let pixel_index = (sample_y * settings.input_exr_width + sample_x) as usize;
    pixel_index
}

fn chunk_in_center_percent_range(c: Chunk, percent: f32, width: i32, height: i32) -> bool {
    let x = in_center_percent_range(c.x, percent, width);
    let y = in_center_percent_range(c.y, percent, height);

    x && y
}

fn in_center_percent_range(value: i32, percent: f32, total: i32) -> bool {
    let one_percent = total / 100;
    let center = total / 2;
    let half_percent = percent * 0.5 * one_percent as f32;

    let min = center as f32 - half_percent;
    let max = center as f32 + half_percent;
    let value = value as f32;

    (value > min) && (value < max)
}
