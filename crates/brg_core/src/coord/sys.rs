use bevy::prelude::{
    error,
    warn,
    Camera,
    Camera3d,
    GlobalTransform,
    InfinitePlane3d,
    MeshRayCast,
    Query,
    RayCastSettings,
    ResMut,
    Vec3,
    Window,
    With,
};
use bevy::window::PrimaryWindow;

use crate::coord::res::{CoordsArea, ResCoords};
use crate::prelude::{V2, V3};

pub fn update_world_coords(
    mut data: ResMut<ResCoords>,
    query_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut mesh_ray_cast: MeshRayCast,
) {
    let Some(window) = query_window.iter().next() else {
        warn!("window not exist, skip coords calculation");
        return;
    };

    for (cam, trm) in &query_cam {
        if !cam.is_active {
            continue;
        }

        let Some(viewport_rect) = cam.logical_viewport_rect() else {
            warn!("can`t get camera viewport rect");
            continue;
        };

        data.screen_ui_width = viewport_rect.width() as u32;
        data.screen_ui_height = viewport_rect.height() as u32;
        data.screen_ui_pos = CoordsArea {
            top_left:     V2::new(viewport_rect.min.x, viewport_rect.min.y),
            top_right:    V2::new(viewport_rect.max.x, viewport_rect.min.y),
            bottom_left:  V2::new(viewport_rect.min.x, viewport_rect.max.y),
            bottom_right: V2::new(viewport_rect.max.x, viewport_rect.max.y),
        };

        data.mouse_ui_pos = match window.cursor_position() {
            Some(window_pos) => V2::from_2d_ui(window_pos),
            None => continue,
        };

        if let Some(pos) = ray_cast(&mut mesh_ray_cast, cam, trm, data.mouse_ui_pos) {
            data.mouse_world_pos = pos;
        }

        if let Some(pos) = ray_cast(&mut mesh_ray_cast, cam, trm, data.screen_ui_pos.top_left) {
            data.screen_world_box.top_left = pos;
        }
        if let Some(pos) = ray_cast(&mut mesh_ray_cast, cam, trm, data.screen_ui_pos.top_right) {
            data.screen_world_box.top_right = pos;
        }
        if let Some(pos) = ray_cast(&mut mesh_ray_cast, cam, trm, data.screen_ui_pos.bottom_left) {
            data.screen_world_box.bottom_left = pos;
        }
        if let Some(pos) = ray_cast(
            &mut mesh_ray_cast,
            cam,
            trm,
            data.screen_ui_pos.bottom_right,
        ) {
            data.screen_world_box.bottom_right = pos;
        }

        break;
    }
}

fn ray_cast(
    ray: &mut MeshRayCast,
    cam: &Camera,
    cam_trm: &GlobalTransform,
    viewport_position: V2,
) -> Option<V2> {
    let ray3d = match cam.viewport_to_world(cam_trm, viewport_position.as_2d_ui()) {
        Ok(ray3d) => ray3d,
        Err(_) => return None,
    };

    let hit_location = match ray.cast_ray(ray3d, &RayCastSettings::default()).first() {
        Some((_, hit)) => V3::from_3d(hit.point),
        None => {
            let Some(dist) = ray3d.intersect_plane(Vec3::ZERO, InfinitePlane3d::default()) else {
                return None;
            };
            V3::from_3d(ray3d.get_point(dist))
        }
    };

    Some(V2::new(hit_location.x, hit_location.y))
}
