use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Context, Result};
use brg_core::prelude::{Block, Tile};

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
        fs::read(input_directory.join("lay0.heightmap.r8")).context("reading file")?;
    let size = file_content.len() as u32;
    let width = size.isqrt();
    let height = width;

    // create image
    let mut pixels_buffer = PixelBuffer::new(width, height, 3);

    // copy to pixel buffer
    let count = width * height;
    let mut tx = 0;
    let mut ty = 0;
    for ind in 0..count as usize {
        let tile = Tile::at(tx, ty);
        pixels_buffer.write_at(tile, (file_content[ind] as f32) / 255.0);

        tx += 1;
        if tx >= width as i32 {
            tx = 0;
            ty += 1;
        }
    }

    // write to file
    let png_file = File::create(input_directory.join("landscape-preview.png"))
        .context("creating preview png file")?;
    let png_buff = &mut BufWriter::new(png_file);

    let mut encoder = png::Encoder::new(png_buff, width, height);
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

    fn write_at(&mut self, px: Tile, data: f32) {
        let data = data.clamp(0.0, 1.0);
        let index = self.index_by_tile(px);
        let (r, g, b) = (data, data, data);

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
