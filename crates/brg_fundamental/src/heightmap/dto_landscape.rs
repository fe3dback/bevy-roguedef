use std::borrow::Cow;
use std::collections::HashMap;

use brg_core::prelude::{
    weighted_fill,
    Area,
    Block,
    BlockChild,
    BlockParent,
    BlockPosition,
    Chunk,
    CntOfAreas,
    T_LIB_CONT_ROW_LEN,
    T_LIB_CONT_SIZE_SQ,
    V2,
};
use brg_scene::prelude::AreaHeights;

#[derive(Default)]
pub(super) struct Landscape {
    pub width:  CntOfAreas,
    pub height: CntOfAreas,

    /// always has width*height elements
    /// in order of [top->bottom; left->right]
    pub areas: Vec<LandscapeArea>,
}

#[derive(Clone)]
pub(super) struct LandscapeArea {
    // key heights for area
    // (center, [TL, TR, BL, BR])
    pub heights: AreaHeights,

    /// affects [chunks]
    pub has_chunks: bool,

    /// has [T_LIB_CONT_SIZE_SQ] elements when [has_chunks]
    /// otherwise will be empty
    /// in order of [top->bottom; left->right]
    pub chunks: Vec<LandscapeChunk>,
}

#[derive(Debug, Clone)]
pub(super) struct LandscapeChunk {
    /// height for every tile in chunk
    /// in order of [top->bottom; left->right]
    pub heights: [f32; T_LIB_CONT_SIZE_SQ],
}

impl Landscape {
    pub(super) fn chunk_heights(&self, chunk: Chunk) -> Option<Cow<[f32; T_LIB_CONT_SIZE_SQ]>> {
        // apply offset to center map, so pos[0.0] will be in [width/2, height/2]
        let offset_x = self.width as usize * Area::size() / 2;
        let offset_y = self.height as usize * Area::size() / 2;
        let chunk = Chunk::at(offset_x as i32, offset_y as i32) + chunk;

        let area = chunk.parent();
        let area_index = self.area_index(area);
        if area_index < 0 || area_index >= (self.width * self.height) as usize {
            return None;
        }

        let area_data = &self.areas[area_index];

        // when has detailed heights
        if area_data.has_chunks {
            let chunk_index = self.chunk_index_inside_area(area, chunk);
            return Some(Cow::Borrowed(&area_data.chunks[chunk_index].heights));
        }

        // otherwise - interpolate from area
        let mut anchor_points = HashMap::<V2, f32>::with_capacity(5);
        anchor_points.insert(area.position_center(), area_data.heights.0);
        anchor_points.insert(area.position_tl(), area_data.heights.1[0]);
        anchor_points.insert(area.position_tr(), area_data.heights.1[1]);
        anchor_points.insert(area.position_bl(), area_data.heights.1[2]);
        anchor_points.insert(area.position_br(), area_data.heights.1[3]);

        Some(Cow::Owned(weighted_fill(
            anchor_points,
            chunk.position_tl(),
        )))
    }

    #[inline(always)]
    fn area_index(&self, a: Area) -> usize {
        (a.y * self.width as i32 + a.x) as usize
    }

    #[inline(always)]
    fn chunk_index_inside_area(&self, area: Area, c: Chunk) -> usize {
        let offset = area.child_elem_tl();
        let rel_pos = c - offset;

        (rel_pos.y * T_LIB_CONT_ROW_LEN as i32 + rel_pos.x) as usize
    }

    #[inline(always)]
    fn area_by_index(&self, index: usize) -> Area {
        let y = index / self.width as usize;
        let x = index % self.height as usize;

        Area::at(x as i32, y as i32)
    }
}
