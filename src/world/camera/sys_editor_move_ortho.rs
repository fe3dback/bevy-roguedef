use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::{
    ButtonInput,
    Camera,
    Commands,
    Entity,
    EventReader,
    KeyCode,
    OrthographicProjection,
    Projection,
    Query,
    Res,
    ResMut,
    Time,
};
use brg_core::prelude::V2;
use brg_fundamental::prelude::CmpTransform2D;

use super::enums::CmpCameraType;
use super::res::ResCameraSettings;

pub fn editor_ortho_wasd_move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&mut CmpTransform2D, &Camera, &CmpCameraType, &Projection)>,
    time: Res<Time>,
    settings: Res<ResCameraSettings>,
) {
    if settings.active != CmpCameraType::EditorTopDownOrthographic {
        return;
    }

    for (mut trm, cam, cam_type, cam_projection) in &mut cameras {
        if !cam.is_active {
            continue;
        }

        if *cam_type != CmpCameraType::EditorTopDownOrthographic {
            continue;
        }

        // movement
        let mut move_vec = V2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            move_vec.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            move_vec.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            move_vec.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            move_vec.x += 1.0;
        }

        move_vec = move_vec.normalize();

        let mut speed = 10.0;
        if keyboard.pressed(KeyCode::ShiftLeft) {
            speed *= 3.0;
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            speed /= 3.0;
        }

        let scale = match cam_projection {
            Projection::Orthographic(ortho) => ortho.scale,
            _ => 1.0,
        };

        trm.position += move_vec * speed * scale * time.delta().as_secs_f32();
    }
}

pub fn editor_ortho_change_scale(
    mut cmd: Commands,
    mut mouse: EventReader<MouseWheel>,
    mut cameras: Query<(Entity, &CmpCameraType, &Projection)>,
    mut settings: ResMut<ResCameraSettings>,
) {
    if settings.active != CmpCameraType::EditorTopDownOrthographic {
        return;
    }

    let mut changed = false;

    for ev in mouse.read() {
        let delta = match ev.unit {
            MouseScrollUnit::Line => ev.y * 100.0,
            MouseScrollUnit::Pixel => ev.y,
        };

        let prev = settings.editor_topdown_mouse_scroll;

        settings.editor_topdown_mouse_scroll =
            (settings.editor_topdown_mouse_scroll - delta).clamp(0.0, 1000.0);

        let next = settings.editor_topdown_mouse_scroll;

        if prev != next {
            changed = true;
        }
    }

    if !changed {
        return;
    }

    let scale_step = (settings.editor_topdown_mouse_scroll / 100.0).floor() as u32;

    for (cam_ent, cam_type, cam_projection) in &mut cameras {
        if *cam_type != CmpCameraType::EditorTopDownOrthographic {
            continue;
        }

        let new_projection = match cam_projection {
            Projection::Orthographic(prev_projection) => {
                Projection::Orthographic(OrthographicProjection {
                    scale: match scale_step {
                        0 => 0.025,
                        1 => 0.100,
                        2 => 0.250,
                        3 => 0.500,
                        4 => 0.750,
                        5 => 1.000,
                        6 => 2.000,
                        7 => 4.000,
                        8 => 8.000,
                        9 => 16.00,
                        _ => 32.00,
                    },
                    ..*prev_projection
                })
            }
            _ => continue,
        };

        cmd.entity(cam_ent).remove::<Projection>();
        cmd.entity(cam_ent).insert(new_projection);
    }
}
