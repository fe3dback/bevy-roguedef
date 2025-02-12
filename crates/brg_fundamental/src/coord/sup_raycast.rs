use bevy::ecs::system::SystemParam;
use bevy::picking::mesh_picking::ray_cast::RayMeshHit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use brg_core::prelude::{V2, V3};

use crate::prelude::CmpMarkerTerrainRayCastSolid;

#[derive(SystemParam)]
pub struct SupRayCastMesh<'w, 's> {
    pub mesh_ray_cast:      MeshRayCast<'w, 's>,
    pub query_cam: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera3d>>,
    pub query_window:       Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub query_terrain_mesh: Query<'w, 's, Entity, With<CmpMarkerTerrainRayCastSolid>>,
}

impl SupRayCastMesh<'_, '_> {
    pub(super) fn viewport(&self) -> Option<Rect> {
        for (cam, _) in &self.query_cam {
            if !cam.is_active {
                continue;
            }

            let Some(viewport_rect) = cam.logical_viewport_rect() else {
                warn!("can`t get camera viewport rect");
                continue;
            };

            return Some(viewport_rect);
        }

        None
    }

    pub(super) fn cursor_pos(&self) -> V2 {
        let Ok(win) = self.query_window.get_single() else {
            return V2::ZERO;
        };

        match win.cursor_position() {
            Some(window_pos) => V2::from_2d_ui(window_pos),
            None => V2::ZERO,
        }
    }

    pub(super) fn ray_cast_from_screen(&mut self, viewport_position: V2) -> V3 {
        for (cam, trm) in &self.query_cam {
            if !cam.is_active {
                continue;
            }

            let ray3d = match cam.viewport_to_world(trm, viewport_position.as_2d_ui()) {
                Ok(ray3d) => ray3d,
                Err(_) => return V3::ZERO,
            };

            let hit = self
                .mesh_ray_cast
                .cast_ray(
                    ray3d,
                    &RayCastSettings {
                        visibility: RayCastVisibility::Visible,
                        filter: &|entity| self.query_terrain_mesh.get(entity).is_ok(),
                        ..default()
                    },
                )
                .first();

            return match hit {
                Some((_, hit)) => V3::from_3d(hit.point),
                None => {
                    let Some(dist) = ray3d.intersect_plane(Vec3::ZERO, InfinitePlane3d::default())
                    else {
                        return V3::ZERO;
                    };
                    V3::from_3d(ray3d.get_point(dist))
                }
            };
        }

        V3::ZERO
    }

    pub fn ray_cast(&mut self, from: V3, to: V3) -> V3 {
        let start = from.as_3d();
        let direction = (to - from).as_3d().normalize();
        let Ok(dir) = Dir3::new(direction) else {
            warn!("can`t get direction from {}", direction);
            return V3::ZERO;
        };

        let ray3d = Ray3d::new(start, dir);

        let hits = self.mesh_ray_cast.cast_ray(
            ray3d,
            &RayCastSettings {
                visibility: RayCastVisibility::Visible,
                filter: &|entity| self.query_terrain_mesh.get(entity).is_ok(),
                ..default()
            },
        );

        match hits.first() {
            Some((_, hit)) => V3::from_3d(hit.point),
            None => {
                let Some(dist) = ray3d.intersect_plane(Vec3::ZERO, InfinitePlane3d::default())
                else {
                    return V3::ZERO;
                };
                V3::from_3d(ray3d.get_point(dist))
            }
        }
    }
}
