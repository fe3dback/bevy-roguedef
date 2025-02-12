use bevy::color::palettes::tailwind::AMBER_500;
use bevy::prelude::Res;
use brg_fundamental::prelude::{Point, ResCoords, SupGizmos};

pub fn sys_debug_draw_world_mouse_pos(mut gz: SupGizmos, coords: Res<ResCoords>) {
    gz.line(
        Point::Rel(coords.mouse_world_pos.with_height(0.0)),
        Point::Rel(coords.mouse_world_pos.with_height(1.0)),
        AMBER_500,
    );
    gz.point(
        Point::Rel(coords.mouse_world_pos.with_height(0.0)),
        AMBER_500,
    );
}
