use brg_core::prelude::T_LIB_CONT_SIZE_SQ;

pub type AreaHeights = (f32, [f32; 4]);

pub struct LevelData {
    pub(super) version:   u8,
    pub(super) name:      String,
    /// level width in areas
    pub(super) width:     u32,
    /// level height in areas
    pub(super) height:    u32,
    /// level landscape height data
    pub(super) landscape: LevelDataLandscape,
}

pub struct LevelDataLandscape {
    /// always has width*height elements
    /// in order of [top->bottom; left->right]
    pub areas: Vec<LevelDataLandscapeArea>,
}

#[derive(Clone)]
pub struct LevelDataLandscapeArea {
    // key heights for area
    // (center, [TL, TR, BL, BR])
    pub heights: AreaHeights,

    /// affects [chunks]
    pub has_chunks: bool,

    /// has [T_LIB_CONT_SIZE_SQ] elements when [has_chunks]
    /// otherwise will be empty
    /// in order of [top->bottom; left->right]
    pub chunks: Vec<LevelDataLandscapeChunk>,
}

#[derive(Debug, Clone)]
pub struct LevelDataLandscapeChunk {
    /// height for every tile in chunk
    /// in order of [top->bottom; left->right]
    pub heights: [f32; T_LIB_CONT_SIZE_SQ],
}

impl LevelData {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            version: 1,
            name,
            width,
            height,
            landscape: LevelDataLandscape::new(width, height),
        }
    }
}

impl LevelDataLandscape {
    fn new(w: u32, h: u32) -> Self {
        Self {
            areas: vec![LevelDataLandscapeArea::new_uninitialized(); (w * h) as usize],
        }
    }
}

impl LevelDataLandscapeArea {
    fn new_uninitialized() -> Self {
        Self {
            heights:    (0.0, [0.0; 4]),
            has_chunks: false,
            chunks:     vec![],
        }
    }

    pub fn new_without_chunks(heights: AreaHeights) -> Self {
        Self {
            has_chunks: false,
            heights,
            chunks: vec![],
        }
    }

    pub fn new_with_chunks(
        heights: AreaHeights,
        chunks: [LevelDataLandscapeChunk; T_LIB_CONT_SIZE_SQ],
    ) -> Self {
        Self {
            has_chunks: true,
            heights,
            chunks: chunks.into(),
        }
    }
}

impl LevelDataLandscapeChunk {
    pub fn new(heights: [f32; T_LIB_CONT_SIZE_SQ]) -> Self {
        Self { heights }
    }
}
