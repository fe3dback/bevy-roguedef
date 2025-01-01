use bevy::color::palettes::basic::{BLUE, GREEN, RED};
use brg_core::prelude::{GizmosX, V2, V3};

pub fn sys_debug_draw_world_origin(mut gz: GizmosX) {
    gz.arrow(V2::ZERO, V2::new(5.0, 0.0), RED);
    gz.arrow(V2::ZERO, V2::new(0.0, 5.0), GREEN);
    gz.arrow_custom_height(V3::ZERO, V3::new(0.0, 0.0, 5.0), BLUE);
}
