use std::borrow::Cow;
use std::collections::HashMap;

use brg_core::prelude::{
    Area,
    Block,
    BlockParent,
    BlockPosition,
    Chunk,
    T_LIB_CONT_ROW_LEN,
    T_LIB_ELEM_INDEX_BL,
    T_LIB_ELEM_INDEX_BR,
    T_LIB_ELEM_INDEX_CENTER,
    T_LIB_ELEM_INDEX_TL,
    T_LIB_ELEM_INDEX_TR,
    V2,
};

use super::data_v1_heights::RawLevelChunkHeights;
use super::interpolate::weighted_fill;

pub struct RawLevelData {
    pub version:       u8,
    pub name:          String,
    pub width_chunks:  u32,
    pub height_chunks: u32,

    pub width_areas:  u32,
    pub height_areas: u32,

    /// all world chunks from [0,0] to [wX, wY]
    ///
    ///     example layout for (width=3, height=2):
    ///       chunks: [0, 1, 2, 3, 4, 5]
    ///       data:
    ///         [0] [1] [2]
    ///         [3] [4] [5]
    pub chunks: Vec<RawLevelChunk>,

    /// all world areas from [0,0] to [wX, wY]
    ///
    ///     example layout for (width=3, height=2):
    ///       chunks: [0, 1, 2, 3, 4, 5]
    ///       data:
    ///         [0] [1] [2]
    ///         [3] [4] [5]
    pub areas: Vec<RawLevelArea>,
}

#[derive(Clone)]
pub struct RawLevelArea {
    pub area:    Area,            // not saved to file
    pub heights: (f32, [f32; 4]), // Center, TL, TR, BL, BR
}

impl RawLevelArea {
    pub fn new(area: Area) -> Self {
        Self {
            area,
            heights: (0.0, [0.0; 4]),
        }
    }
}

#[derive(Clone)]
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
        let (width_areas, height_areas) = (
            (width_chunks as f32 / T_LIB_CONT_ROW_LEN as f32).ceil() as u32,
            (height_chunks as f32 / T_LIB_CONT_ROW_LEN as f32).ceil() as u32,
        );

        Self {
            version: 1,
            name: name.into(),
            width_chunks,
            height_chunks,
            width_areas,
            height_areas,
            areas: vec![RawLevelArea::new(Area::default()); (width_areas * height_areas) as usize],
            chunks: vec![RawLevelChunk::default(); (width_chunks * height_chunks) as usize],
        }
    }

    #[inline(always)]
    pub fn chunk_index(&self, c: Chunk) -> usize {
        (c.y * self.width_chunks as i32 + c.x) as usize
    }

    #[inline(always)]
    pub fn area_index(&self, a: Area) -> usize {
        (a.y * self.width_areas as i32 + a.x) as usize
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
    pub fn area_write(&mut self, data: RawLevelArea) {
        let index = self.area_index(data.area);
        self.areas[index] = data;
    }

    #[inline(always)]
    pub fn chunk_read(&self, c: Chunk) -> Cow<RawLevelChunk> {
        let index = self.chunk_index(c);
        let data = &self.chunks[index];

        match data.heights {
            RawLevelChunkHeights::Empty => {
                let mut data = data.clone();

                let area = c.parent();
                let area_index = self.area_index(area);
                let area_data = &self.areas[area_index];

                let mut key_points: HashMap<V2, f32> = HashMap::with_capacity(5);
                key_points.insert(
                    area.position_center() - c.position_center(),
                    area_data.heights.0,
                );
                key_points.insert(area.position_tl() - c.position_tl(), area_data.heights.1[0]);
                key_points.insert(area.position_tr() - c.position_tr(), area_data.heights.1[1]);
                key_points.insert(area.position_bl() - c.position_bl(), area_data.heights.1[2]);
                key_points.insert(area.position_br() - c.position_br(), area_data.heights.1[3]);
                let interpolated = weighted_fill(key_points);

                data.heights =
                    RawLevelChunkHeights::Minimal(interpolated[T_LIB_ELEM_INDEX_CENTER], [
                        interpolated[T_LIB_ELEM_INDEX_TL],
                        interpolated[T_LIB_ELEM_INDEX_TR],
                        interpolated[T_LIB_ELEM_INDEX_BL],
                        interpolated[T_LIB_ELEM_INDEX_BR],
                    ]);

                Cow::Owned(data)
            }
            _ => Cow::Borrowed(&data),
        }
    }
}
