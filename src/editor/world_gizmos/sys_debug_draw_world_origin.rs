use bevy::color::palettes::tailwind::AMBER_500;
use bevy::prelude::Res;
use brg_fundamental::prelude::{GizmosX, ResCoords};

pub fn sys_debug_draw_world_mouse_pos(mut gz: GizmosX, coords: Res<ResCoords>) {
    gz.line_custom_height(
        coords.mouse_world_pos.with_height(0.0),
        coords.mouse_world_pos.with_height(1.0),
        AMBER_500,
    );
    gz.point_custom_height(coords.mouse_world_pos.with_height(0.0), AMBER_500);
}
