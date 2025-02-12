use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use binary_rw::{BinaryWriter, Endian, MemoryStream};
use brg_core::prelude::{remap, Block, Chunk, Tile};
use exr::prelude::{FlatSamples, IntegerBounds};

#[derive(Debug)]
pub struct Importer {
    samples: FlatSamples,

    input_exr_width:  u32, // real exr image width
    input_exr_height: u32, // real exr image height
    input_min_value:  f32, // min pixel grayscale value (aka height/saturation) of all exr pixels
    input_max_value:  f32, // max pixel grayscale value (aka height/saturation) of all exr pixels

    width:          u32, // export width
    height:         u32, // export height
    scale_factor_x: f32, // how many tiles will be mapped to single exr pixel by x
    scale_factor_y: f32, // how many tiles will be mapped to single exr pixel bv y
}

pub enum MapSize {
    S1024,
    S2048,
    S4096,
}

impl MapSize {
    pub fn bevy_units(&self) -> u32 {
        match self {
            MapSize::S1024 => 1024,
            MapSize::S2048 => 2048,
            MapSize::S4096 => 4096,
        }
    }
}

const MAP_SIZE: MapSize = MapSize::S2048;
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
    let mut data = MemoryStream::from(vec![0; ((importer.width * importer.height) * 2) as usize]);
    let mut writer = BinaryWriter::new(&mut data, Endian::Little);

    for y in 0..importer.height {
        for x in 0..importer.width {
            let tile = Tile::at(x as i32, y as i32);
            let height = importer.sample(tile);

            writer.write_f32(height)?;
        }
    }

    // write data to bevy output
    {
        let out_path = out_directory.join("lay0.heightmap.r32");
        fs::create_dir_all(&out_path.parent().unwrap()).context("creating map directories")?;

        let mut file = File::create(out_path).context("failed create area file")?;
        let content: Vec<u8> = data.into();
        file.write_all(&content).context("writing bytes")?;
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

    let input_size = Importer::found_closest_square_bound(in_bounds);
    let want_size = MAP_SIZE;

    let map_width_m = want_size.bevy_units();
    let map_height_m = want_size.bevy_units();

    let (input_min_value, input_max_value) = Importer::find_min_max_height(&samples);

    let scale = want_size.bevy_units() as f32 / input_size.bevy_units() as f32;

    Importer {
        samples,

        input_exr_width,
        input_exr_height,
        input_min_value,
        input_max_value,

        width: map_width_m,
        height: map_height_m,
        scale_factor_x: scale,
        scale_factor_y: scale,
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
    fn found_closest_square_bound(in_bounds: IntegerBounds) -> MapSize {
        let (input_exr_width, input_exr_height) =
            (in_bounds.size.x() as u32, in_bounds.size.y() as u32);

        if input_exr_width != input_exr_height {
            panic!("only square input is supported")
        }

        if input_exr_height >= 4096 {
            return MapSize::S4096;
        }

        if input_exr_height >= 2048 {
            return MapSize::S2048;
        }

        if input_exr_height >= 1024 {
            return MapSize::S1024;
        }

        panic!("input exr resolution is very small")
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
