use std::collections::HashMap;

use anyhow::{Context, Result};
use brg_core::prelude::{Block, Chunk, V2};
use serde::{Deserialize, Serialize};
use serde_binary::binary_stream::Endian;

#[derive(Serialize, Deserialize)]
pub struct RawLevelData {
    pub name:          String,
    pub width_chunks:  u32,
    pub height_chunks: u32,

    /// all world chunks from [0,0] to [wX, wY]
    ///
    ///     example layout for (width=3, height=2):
    ///       chunks: [0, 1, 2, 3, 4, 5]
    ///       data:
    ///         [0] [1] [2]
    ///         [3] [4] [5]
    pub chunks: Vec<RawLevelChunk>,
}

/// Layout:
///
///     Splat  : 1 [X]
///     Minimal: 1 [X] + 4 [#]
///     Lite   : 1 [X] + 4 [#] + 12 [L]
///     Full   : 225 [.] (all vertices)
///
///     [#] [ ] [ ] [ ] [L] [ ] [ ] (|) [ ] [ ] [L] [ ] [ ] [ ] [#]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [L] [ ] [ ] [ ] [L] [ ] [ ] (|) [ ] [ ] [L] [ ] [ ] [ ] [L]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [-] [-] [-] [-] [-] [-] [-] [X] [-] [-] [-] [-] [-] [-] [-]  
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [L] [ ] [ ] [ ] [L] [ ] [ ] (|) [ ] [ ] [L] [ ] [ ] [ ] [L]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [ ] [ ] [ ] [ ] [ ] [ ] [ ] (|) [ ] [ ] [ ] [ ] [ ] [ ] [ ]
///     [#] [ ] [ ] [ ] [L] [ ] [ ] (|) [ ] [ ] [L] [ ] [ ] [ ] [#]
///
///
#[derive(Serialize, Deserialize, Clone)]
pub enum RawLevelChunkHeights {
    // LEVEL: 4
    // all chunk vertices have single height value
    // useful for distant terrain chunks (not playable)
    Splat(f32),

    // LEVEL: 3
    // only primary chunk vertices (CENTER, TL, TR, BL, BR)
    // all chunk tiles must be calculated(lerp) from this vertices
    Minimal(f32, [f32; 4]),

    // LEVEL: 2
    // primary chunk vertices (CENTER, TL, TR, BL, BR)
    // with additional support vertices
    // all chunk tiles must be calculated(lerp) from this vertices
    Lite(f32, [f32; 4], [f32; 12]),

    // LEVEL: 1
    // all chunk vertices
    Full(Vec<f32>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawLevelChunk {
    pub heights: RawLevelChunkHeights,
}

impl Default for RawLevelChunk {
    fn default() -> Self {
        Self {
            heights: RawLevelChunkHeights::Splat(0.0),
        }
    }
}

impl RawLevelData {
    pub fn new(name: impl Into<String>, width_chunks: u32, height_chunks: u32) -> Self {
        Self {
            name: name.into(),
            width_chunks,
            height_chunks,
            chunks: vec![RawLevelChunk::default(); (width_chunks * height_chunks) as usize],
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_binary::to_vec(&self, Endian::Little)
            .context("failed encode raw level data into bytes")
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        serde_binary::from_vec(data, Endian::Little)
            .context("failed decode raw level data into bytes")
    }

    #[inline(always)]
    pub fn chunk_index(&self, c: Chunk) -> usize {
        (c.y * self.width_chunks as i32 + c.x) as usize
    }

    #[inline(always)]
    pub fn chunk_by_index(&self, index: usize) -> Chunk {
        let y = index / self.width_chunks as usize;
        let x = index % self.width_chunks as usize;

        Chunk::at(x as i32, y as i32)
    }

    #[inline(always)]
    pub fn chunk_write(&mut self, c: Chunk, data: RawLevelChunk) {
        let index = self.chunk_index(c);
        self.chunks[index] = data;
    }

    #[inline(always)]
    pub fn chunk_read(&mut self, c: Chunk) -> &RawLevelChunk {
        let index = self.chunk_index(c);
        &self.chunks[index]
    }
}

const CHUNK_SIZE_SQ: usize = Chunk::size() as usize * Chunk::size() as usize;
type chunk_heights = [f32; CHUNK_SIZE_SQ];

impl RawLevelChunkHeights {
    pub fn interpolate(&self) -> chunk_heights {
        let chunk_center = (Chunk::size() as f32 / 2.0).floor();
        let chunk_min: f32 = 0.0;
        let chunk_max = Chunk::size() as f32 - 1.0;

        match self {
            RawLevelChunkHeights::Full(data) => {
                let mut raw: chunk_heights = [0.0; CHUNK_SIZE_SQ];
                for (ind, value) in data.iter().enumerate() {
                    raw[ind] = *value;
                }

                raw
            }
            RawLevelChunkHeights::Lite(center, verts, support) => {
                let mut key_points: HashMap<V2, f32> = HashMap::new();
                key_points.insert(V2::new(chunk_center, chunk_center), *center);

                key_points.insert(V2::new(chunk_min, chunk_min), verts[0]); // tl
                key_points.insert(V2::new(chunk_max, chunk_min), verts[1]); // tr
                key_points.insert(V2::new(chunk_min, chunk_max), verts[2]); // bl
                key_points.insert(V2::new(chunk_max, chunk_max), verts[3]); // br

                // row 0 (first)
                key_points.insert(V2::new(chunk_min + 4.0, chunk_min), support[0]);
                key_points.insert(V2::new(chunk_max - 4.0, chunk_min), support[1]);

                // row 4
                key_points.insert(V2::new(chunk_min, chunk_min + 4.0), support[2]);
                key_points.insert(V2::new(chunk_min + 4.0, chunk_min + 4.0), support[3]);
                key_points.insert(V2::new(chunk_max - 4.0, chunk_min + 4.0), support[4]);
                key_points.insert(V2::new(chunk_max, chunk_min + 4.0), support[5]);

                // row 10
                key_points.insert(V2::new(chunk_min, chunk_max - 4.0), support[6]);
                key_points.insert(V2::new(chunk_min + 4.0, chunk_max - 4.0), support[7]);
                key_points.insert(V2::new(chunk_max - 4.0, chunk_max - 4.0), support[8]);
                key_points.insert(V2::new(chunk_max, chunk_max - 4.0), support[9]);

                // row 14 (last)
                key_points.insert(V2::new(chunk_min + 4.0, chunk_max), support[10]);
                key_points.insert(V2::new(chunk_max - 4.0, chunk_max), support[11]);

                weighted_fill(key_points)
            }
            RawLevelChunkHeights::Minimal(center, verts) => {
                let mut key_points: HashMap<V2, f32> = HashMap::new();
                key_points.insert(V2::new(chunk_center, chunk_center), *center);

                key_points.insert(V2::new(chunk_min, chunk_min), verts[0]); // tl
                key_points.insert(V2::new(chunk_max, chunk_min), verts[1]); // tr
                key_points.insert(V2::new(chunk_min, chunk_max), verts[2]); // bl
                key_points.insert(V2::new(chunk_max, chunk_max), verts[3]); // br

                weighted_fill(key_points)
            }
            RawLevelChunkHeights::Splat(height) => [*height; CHUNK_SIZE_SQ],
        }
    }
}

pub fn weighted_fill(key_points: HashMap<V2, f32>) -> [f32; CHUNK_SIZE_SQ] {
    let mut result = [0.0; CHUNK_SIZE_SQ];

    let mut tile_x: i32 = 0;
    let mut tile_y: i32 = 0;

    fn dist(p1: V2, p2: V2) -> f32 {
        let dx = (p1.x as f32 - p2.x as f32).abs();
        let dy = (p1.y as f32 - p2.y as f32).abs();
        (dx.powi(2) + dy.powi(2)).sqrt()
    };

    for ind in 0..CHUNK_SIZE_SQ {
        let tile = V2::new(tile_x as f32, tile_y as f32);

        // calculate weights
        let mut total_value: f32 = 0.0;
        let mut total_weight = 0.0;

        for (key_tile, key_value) in &key_points {
            let distance = dist(tile, *key_tile);
            match distance < f32::EPSILON {
                true => {
                    total_value = *key_value;
                    total_weight = 1.0;
                    break;
                }
                false => {
                    let weight = 1.0 / distance;
                    total_value += key_value * weight;
                    total_weight += weight;
                }
            };
        }

        result[ind] = match total_weight < f32::EPSILON {
            true => 0.0,
            false => total_value / total_weight,
        };

        // move cursor
        tile_x += 1;

        if tile_x >= Chunk::size() {
            tile_x = 0;
            tile_y += 1;
        }
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn weighted_fill_test() {
        let mut key_points: HashMap<V2, f32> = HashMap::new();
        key_points.insert(V2::new(0.0, 0.0), 0.25);
        key_points.insert(V2::new(14.0, 0.0), 1.0);
        key_points.insert(V2::new(0.0, 14.0), 0.0);
        key_points.insert(V2::new(14.0, 14.0), 1.0);
        key_points.insert(V2::new(7.0, 7.0), 0.0);

        let result = weighted_fill(key_points);

        let mut tile_x: i32 = 0;

        for val in &result {
            print!("{}", match *val {
                f32::MIN..0.25 => "░░",
                0.25..0.5 => "▒▒",
                0.5..0.75 => "▓▓",
                0.75..f32::MAX => "██",
                _ => "?",
            });

            tile_x += 1;
            if tile_x >= Chunk::size() {
                tile_x = 0;
                println!();
            }
        }

        println!("----");

        for val in &result {
            print!(" {:.2} |", *val);

            tile_x += 1;
            if tile_x >= Chunk::size() {
                tile_x = 0;
                println!();
            }
        }
    }
}
