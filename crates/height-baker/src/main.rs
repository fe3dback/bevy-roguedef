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
    T_LIB_CONT_ROW_LEN,
    T_LIB_CONT_SIZE_SQ,
};
use brg_scene::prelude::{AreaHeights, LevelData, LevelDataLandscapeArea, LevelDataLandscapeChunk};
use exr::prelude::{FlatSamples, IntegerBounds};

#[derive(Debug)]
pub struct Importer {
    samples: FlatSamples,

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

const MAP_SIZE_M: Meter = 4096.0; // 4 km^2
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
    let importer = new_importer(bounds, samples);
    println!("[LOADED] settings: {:?}", importer);

    // build areas
    let mut level = LevelData::new(
        MAP_NAME.to_string(),
        importer.areas_width,
        importer.areas_height,
    );

    for abs_area_y in 0..importer.areas_height as i32 {
        for abs_area_x in 0..importer.areas_width as i32 {
            let area = Area::at(abs_area_x, abs_area_y);

            let mut area_has_chunks = false;
            let mut area_heights = AreaHeights::default();
            let mut area_chunks: Vec<LevelDataLandscapeChunk> =
                Vec::with_capacity(T_LIB_CONT_SIZE_SQ);

            for chunk in &area.child_range() {
                // calculate chunk heights
                {
                    let mut heights = [0.0; T_LIB_CONT_SIZE_SQ];

                    for (ind, tile) in chunk.child_range().into_iter().enumerate() {
                        heights[ind] = importer.sample(tile);
                    }

                    area_chunks.push(LevelDataLandscapeChunk::new(heights));
                }

                // update area importance
                {
                    if chunk_in_center_percent_range(
                        chunk,
                        35.0,
                        importer.chunks_width,
                        importer.chunks_height,
                    ) {
                        area_has_chunks = true;
                    }
                }

                // update area key points
                {
                    if area.child_elem_center() == chunk {
                        area_heights.0 = importer.sample(chunk.child_elem_center());
                    }
                    if area.child_elem_tl() == chunk {
                        area_heights.1[0] = importer.sample(chunk.child_elem_tl());
                    }
                    if area.child_elem_tr() == chunk {
                        area_heights.1[1] = importer.sample(chunk.child_elem_tr() + Tile::at(1, 0));
                    }
                    if area.child_elem_bl() == chunk {
                        area_heights.1[2] = importer.sample(chunk.child_elem_bl() + Tile::at(0, 1));
                    }
                    if area.child_elem_br() == chunk {
                        area_heights.1[3] = importer.sample(chunk.child_elem_br() + Tile::at(1, 1));
                    }
                }
            }

            let area_data = match area_has_chunks {
                true => LevelDataLandscapeArea::new_with_chunks(
                    area_heights,
                    area_chunks.try_into().unwrap(),
                ),
                false => LevelDataLandscapeArea::new_without_chunks(area_heights),
            };

            level.landscape_add_area(area, area_data);
        }
    }

    // write data to files
    {
        let out_path = out_directory.join("x.land.bin");
        fs::create_dir_all(&out_path.parent().unwrap()).context("creating map directories")?;

        let mut file = File::create(out_path).context("failed create area file")?;
        let file_content = level.encode().context("failed serialize level data")?;
        file.write_all(&file_content).context("writing bytes")?;
    }

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

fn new_importer(in_bounds: IntegerBounds, samples: FlatSamples) -> Importer {
    let (input_exr_width, input_exr_height) =
        (in_bounds.size.x() as u32, in_bounds.size.y() as u32);

    let (input_exr_closest_dst_width, input_exr_closest_dst_height) =
        Importer::found_closest_square_bound(in_bounds);

    let map_width_m = MAP_SIZE_M.floor() as u32;
    let map_height_m = MAP_SIZE_M.floor() as u32;

    let (input_min_value, input_max_value) = Importer::find_min_max_height(&samples);

    let areas_width = (map_width_m as f32 / Area::size_m().x).floor() as u32;
    let areas_height = (map_height_m as f32 / Area::size_m().y).floor() as u32;

    let chunks_width = areas_width * T_LIB_CONT_ROW_LEN as u32;
    let chunks_height = areas_height * T_LIB_CONT_ROW_LEN as u32;

    Importer {
        samples,

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

impl Importer {
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

    #[inline(always)]
    fn calculate_pixel_index(&self, tile: Tile) -> usize {
        let sample_x = (tile.x as f32 / self.scale_factor_x as f32).floor() as u32;
        let sample_y = (tile.y as f32 / self.scale_factor_y as f32).floor() as u32;

        let pixel_index = (sample_y * self.input_exr_width + sample_x) as usize;
        pixel_index
    }

    fn sample(&self, tile: Tile) -> f32 {
        let pixel_index = self.calculate_pixel_index(tile);
        let sample = self.samples.value_by_flat_index(pixel_index);

        remap(
            self.input_min_value,
            self.input_max_value,
            0.0,
            1.0,
            sample.to_f32(),
        )
    }
}

fn chunk_in_center_percent_range(c: Chunk, percent: f32, width: u32, height: u32) -> bool {
    let x = in_center_percent_range(c.x, percent, width);
    let y = in_center_percent_range(c.y, percent, height);

    x && y
}

fn in_center_percent_range(value: i32, percent: f32, total: u32) -> bool {
    let one_percent = total / 100;
    let center = total / 2;
    let half_percent = percent * 0.5 * one_percent as f32;

    let min = center as f32 - half_percent;
    let max = center as f32 + half_percent;
    let value = value as f32;

    (value > min) && (value < max)
}
