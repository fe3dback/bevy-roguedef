use bevy::color::palettes::basic::{BLUE, GREEN, RED};
use brg_core::prelude::{V2, V3};
use brg_fundamental::prelude::{Point, SupGizmos};

pub fn sys_debug_draw_world_origin(mut gz: SupGizmos) {
    gz.arrow(Point::Abs(V2::ZERO), Point::Abs(V2::new(5.0, 0.0)), RED);
    gz.arrow(Point::Abs(V2::ZERO), Point::Abs(V2::new(0.0, 5.0)), GREEN);
    gz.arrow(
        Point::Abs(V3::ZERO),
        Point::Abs(V3::new(0.0, 0.0, 5.0)),
        BLUE,
    );
}
