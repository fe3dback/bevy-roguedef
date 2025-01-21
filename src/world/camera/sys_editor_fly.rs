use std::f32::consts;

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::{
    ButtonInput,
    Camera,
    DetectChanges,
    EulerRot,
    EventReader,
    KeyCode,
    MouseButton,
    Quat,
    Query,
    Res,
    ResMut,
    Time,
    Transform,
    Vec2,
    Vec3,
    Window,
    With,
};
use bevy::window::{CursorGrabMode, PrimaryWindow};

use super::enums::CmpCameraType;
use super::res::ResCameraSettings;

pub fn editor_fly_toggle_mouse_lock(
    mut settings: ResMut<ResCameraSettings>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if settings.active != CmpCameraType::EditorFly {
        return;
    }

    if mouse.just_released(MouseButton::Right) {
        settings.editor_fly_grab_active = false;
        return;
    }

    if mouse.just_pressed(MouseButton::Right) {
        settings.editor_fly_grab_active = true;
        return;
    }
}

pub fn editor_fly_lock_cursor(
    settings: Res<ResCameraSettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }

    let Ok(mut window) = primary_window.get_single_mut() else {
        return;
    };

    match settings.editor_fly_grab_active {
        true => {
            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
        }
        false => {
            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = CursorGrabMode::None;
        }
    };
}

pub fn editor_fly_look_and_move(
    mut mouse_motion: EventReader<MouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseWheel>,
    mut cameras: Query<(&mut Transform, &Camera, &CmpCameraType)>,
    mut settings: ResMut<ResCameraSettings>,
    time: Res<Time>,
) {
    if !settings.editor_fly_grab_active {
        return;
    }

    for ev in mouse.read() {
        let delta = match ev.unit {
            MouseScrollUnit::Line => ev.y * 100.0,
            MouseScrollUnit::Pixel => ev.y,
        };

        let delta = delta * 0.01;
        settings.editor_fly_speed_mul += delta;
        settings.editor_fly_speed_mul = settings.editor_fly_speed_mul.clamp(0.5, 16.0);
    }

    for (mut trm, cam, cam_type) in &mut cameras {
        if !cam.is_active {
            continue;
        }

        if *cam_type != CmpCameraType::EditorFly {
            continue;
        }

        // look around
        let motion = mouse_motion.read().fold(Vec2::ZERO, |acc, m| acc + m.delta);
        let (yaw, pitch, _) = trm.rotation.to_euler(EulerRot::YXZ);
        trm.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw - settings.editor_fly_look_sensitivity.x * motion.x * time.delta().as_secs_f32(),
            (pitch
                - settings.editor_fly_look_sensitivity.y * motion.y * time.delta().as_secs_f32())
            .clamp(-consts::FRAC_PI_3, consts::FRAC_PI_3),
            0.0,
        );

        // movement
        let mut move_vec = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            move_vec += trm.forward() * 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            move_vec += trm.back() * 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            move_vec += trm.left() * 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            move_vec += trm.right() * 1.0;
        }

        let mut speed = 10.0;
        if keyboard.pressed(KeyCode::ShiftLeft) {
            speed *= 3.0;
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            speed /= 3.0;
        }

        trm.translation +=
            move_vec * speed * settings.editor_fly_speed_mul * time.delta().as_secs_f32();
    }
}
