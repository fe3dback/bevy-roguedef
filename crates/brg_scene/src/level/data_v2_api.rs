use std::borrow::Cow;
use std::collections::HashMap;

use brg_core::prelude::{
    Area,
    Block,
    BlockChild,
    BlockParent,
    BlockPosition,
    Chunk,
    T_LIB_CONT_ROW_LEN,
    T_LIB_CONT_SIZE_SQ,
    V2,
};

use super::data_v2::{LevelData, LevelDataLandscapeArea};
use super::interpolate::weighted_fill;

impl LevelData {
    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.width
    }

    pub fn landscape_add_area(&mut self, area: Area, data: LevelDataLandscapeArea) {
        let index = self.area_index(area);
        self.landscape.areas[index] = data;
    }

    pub fn landscape_chunk_heights(&self, chunk: Chunk) -> Cow<[f32; T_LIB_CONT_SIZE_SQ]> {
        let area = chunk.parent();
        let area_index = self.area_index(area);
        let area_data = &self.landscape.areas[area_index];

        // when has detailed heights
        if area_data.has_chunks {
            let chunk_index = self.chunk_index_inside_area(area, chunk);
            return Cow::Borrowed(&area_data.chunks[chunk_index].heights);
        }

        // otherwise - interpolate from area
        let mut anchor_points = HashMap::<V2, f32>::with_capacity(5);
        anchor_points.insert(area.position_center(), area_data.heights.0);
        anchor_points.insert(area.position_tl(), area_data.heights.1[0]);
        anchor_points.insert(area.position_tr(), area_data.heights.1[1]);
        anchor_points.insert(area.position_bl(), area_data.heights.1[2]);
        anchor_points.insert(area.position_br(), area_data.heights.1[3]);

        Cow::Owned(weighted_fill(anchor_points, chunk.position_tl()))
    }

    #[inline(always)]
    pub fn area_index(&self, a: Area) -> usize {
        (a.y * self.width as i32 + a.x) as usize
    }

    #[inline(always)]
    fn chunk_index_inside_area(&self, area: Area, c: Chunk) -> usize {
        let offset = area.child_elem_tl();
        let rel_pos = c - offset;

        (rel_pos.y * T_LIB_CONT_ROW_LEN as i32 + rel_pos.x) as usize
    }

    #[inline(always)]
    pub fn area_by_index(&self, index: usize) -> Area {
        let y = index / self.width as usize;
        let x = index % self.height as usize;

        Area::at(x as i32, y as i32)
    }
}
