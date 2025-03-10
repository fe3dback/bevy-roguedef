use bevy::prelude::{warn, ResMut};
use brg_core::prelude::V2;

use crate::prelude::{CoordsArea, ResCoords, SupRayCastMesh};

// todo: update active camera
// todo: save raycast of frustum origin to res

pub fn update_world_coords(mut data: ResMut<ResCoords>, mut cast: SupRayCastMesh) {
    let Some(viewport_rect) = cast.viewport() else {
        warn!("can`t get camera viewport rect");
        return;
    };

    data.screen_ui_width = viewport_rect.width() as u32;
    data.screen_ui_height = viewport_rect.height() as u32;
    data.screen_ui_pos = CoordsArea {
        top_left:     V2::new(viewport_rect.min.x, viewport_rect.min.y),
        top_right:    V2::new(viewport_rect.max.x, viewport_rect.min.y),
        bottom_left:  V2::new(viewport_rect.min.x, viewport_rect.max.y),
        bottom_right: V2::new(viewport_rect.max.x, viewport_rect.max.y),
    };

    data.mouse_ui_pos = cast.cursor_pos();
    data.mouse_world_pos = cast.ray_cast_from_screen(data.mouse_ui_pos).xy();
    data.screen_world_box.top_left = cast.ray_cast_from_screen(data.screen_ui_pos.top_left).xy();
    data.screen_world_box.top_right = cast.ray_cast_from_screen(data.screen_ui_pos.top_right).xy();
    data.screen_world_box.bottom_left = cast
        .ray_cast_from_screen(data.screen_ui_pos.bottom_left)
        .xy();
    data.screen_world_box.bottom_right = cast
        .ray_cast_from_screen(data.screen_ui_pos.bottom_right)
        .xy();
}
