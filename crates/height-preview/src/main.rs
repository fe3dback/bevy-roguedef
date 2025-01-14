use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Context, Result};
use brg_core::prelude::{Block, BlockChild, Chunk, Tile};
use brg_scene::prelude::{RawLevelChunk, RawLevelChunkHeights, RawLevelData};

enum OutputChannelType {
    Splat,
    Minimal,
    Lite,
    Full,
}

struct PixelBuffer {
    width:    u32,
    height:   u32,
    channels: u8,
    buffer:   Vec<u8>,
}

pub fn main() -> Result<()> {
    let input_directory: &Path =
        Path::new("/home/neo/code/fe3dback/bevy-roguedef/assets/maps/example");

    let file_content =
        fs::read(input_directory.join("world.landscape.bin")).context("reading file")?;
    let data = RawLevelData::from_bytes(file_content).context("decode bin")?;

    // build pixels buffer
    let png_width = data.width_chunks * Chunk::size() as u32;
    let png_height = data.height_chunks * Chunk::size() as u32;

    let mut chunk_x: i32 = 0;
    let mut chunk_y: i32 = 0;

    let mut pixels_buffer = PixelBuffer::new(png_width, png_height, 3);

    for chunk_data in data.chunks {
        let chunk = Chunk::at(chunk_x, chunk_y);
        pixels_buffer.write_chunk(chunk, &chunk_data);

        chunk_x += 1;
        if chunk_x >= data.width_chunks as i32 {
            chunk_x = 0;
            chunk_y += 1;
        }
    }

    // write to file
    let png_file =
        File::create(input_directory.join("preview.png")).context("creating preview png file")?;
    let png_buff = &mut BufWriter::new(png_file);

    let mut encoder = png::Encoder::new(png_buff, png_width, png_height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut png_writer = encoder.write_header().context("create png writer")?;
    png_writer
        .write_image_data(&pixels_buffer.buffer)
        .context("write data to png file")?;
    png_writer.finish().context("finish png file")?;

    Ok(())
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32, channels: u8) -> Self {
        let size = (width * height * channels as u32) as usize;
        let mut buff = Vec::with_capacity(size);

        // init with zero data
        for _ in 0..size {
            buff.push(0);
        }

        Self {
            width,
            height,
            channels,
            buffer: buff,
        }
    }

    pub fn write_chunk(&mut self, c: Chunk, data: &RawLevelChunk) {
        let heights = data.heights.interpolate();

        for (ind, tile) in c.child_range().into_iter().enumerate() {
            let half = (Chunk::size() as f32 / 2.0).floor() as i32;
            let px = tile + Tile::at(half, half);

            let height = heights[ind];
            self.write_at(px, height, match data.heights {
                RawLevelChunkHeights::Full(_) => OutputChannelType::Full,
                RawLevelChunkHeights::Lite(_, _, _) => OutputChannelType::Lite,
                RawLevelChunkHeights::Minimal(_, _) => OutputChannelType::Minimal,
                RawLevelChunkHeights::Splat(_) => OutputChannelType::Splat,
            })
        }
    }

    fn write_at(&mut self, px: Tile, data: f32, ch: OutputChannelType) {
        let data = data.clamp(0.0, 1.0);

        let index = self.index_by_tile(px);
        let (r, g, b) = match ch {
            OutputChannelType::Splat => (data, 0.0, 0.0),
            OutputChannelType::Minimal => (0.0, 0.0, data),
            OutputChannelType::Lite => (0.0, data, 0.0),
            OutputChannelType::Full => (data, data, data),
        };

        self.buffer[index] = (r * 255.0) as u8;
        self.buffer[index + 1] = (g * 255.0) as u8;
        self.buffer[index + 2] = (b * 255.0) as u8;
    }

    fn index_by_tile(&self, px: Tile) -> usize {
        let x = px.x as usize;
        let y = px.y as usize;
        let width = self.width as usize;

        (y * (width * self.channels as usize)) + (x * self.channels as usize)
    }
}
