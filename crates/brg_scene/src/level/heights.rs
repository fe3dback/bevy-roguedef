use brg_core::prelude::{
    T_LIB_ELEM_INDEX_BL,
    T_LIB_ELEM_INDEX_BR,
    T_LIB_ELEM_INDEX_CENTER,
    T_LIB_ELEM_INDEX_TL,
    T_LIB_ELEM_INDEX_TR,
};
use serde::{Deserialize, Serialize};

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
    // LEVEL: 5
    // all chunk heights will be interpolated from parent area
    Empty,

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

impl RawLevelChunkHeights {
    // return actual or interpolated key points heights
    // in order: Center, TL, TR, BL, BR
    pub fn interpolate_key_points(&self) -> (f32, [f32; 4]) {
        match self {
            RawLevelChunkHeights::Empty => (0.0, [0.0; 4]),
            RawLevelChunkHeights::Splat(f) => (*f, [*f; 4]),
            RawLevelChunkHeights::Minimal(center, corners) => (*center, *corners),
            RawLevelChunkHeights::Lite(center, corners, _) => (*center, *corners),
            RawLevelChunkHeights::Full(heights) => (heights[T_LIB_ELEM_INDEX_CENTER], [
                heights[T_LIB_ELEM_INDEX_TL],
                heights[T_LIB_ELEM_INDEX_TR],
                heights[T_LIB_ELEM_INDEX_BL],
                heights[T_LIB_ELEM_INDEX_BR],
            ]),
        }
    }
}
