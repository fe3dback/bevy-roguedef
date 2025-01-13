use anyhow::{Context, Result};
use brg_core::prelude::{Block, Chunk};
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
