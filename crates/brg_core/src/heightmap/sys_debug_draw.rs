use bevy::color::palettes::tailwind::GRAY_900;
use bevy::prelude::Res;

use crate::prelude::{GizmosX, Range, ResHeightmap};

// todo: delete me
pub fn debug_draw_grid(mut gz: GizmosX, hm: Res<ResHeightmap>) {
    let range = Range::new(-10, -10, 10, 10);
    for tile in &range {
        gz.point(tile.position().with_height(0.05), GRAY_900);
    }
}
