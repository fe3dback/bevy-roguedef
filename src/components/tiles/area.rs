// Area is 2D array of chunks
// Area contain exactly 15x15 chunks (225 total)
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Area {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Area {
    #[inline]
    pub fn at(x: i32, y: i32) -> Area {
        Area { x, y }
    }
}