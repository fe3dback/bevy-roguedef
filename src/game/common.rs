use {
    crate::components::lib::V2,
    bevy::{
        prelude::{
            Camera,
            Commands,
            Component,
            DespawnRecursiveExt,
            Entity,
            GlobalTransform,
            Query,
            Reflect,
            Res,
            ResMut,
            Resource,
            Time,
            Window,
            With,
        },
        window::PrimaryWindow,
    },
    rand_chacha::ChaCha8Rng,
};

#[derive(Resource)]
pub struct ResRandomSource {
    pub rnd: ChaCha8Rng,
}

#[derive(Component, Reflect)]
pub struct CmpMainCamera {}

#[derive(Component, Reflect)]
pub struct CmpTimeToLife {
    pub seconds_left: f32,
}

#[derive(Resource, Reflect, Default)]
pub struct ResMouse {
    pub screen_pos: V2,
    pub world_pos:  V2,
}

pub fn remove_expired_ttl_entities(
    mut cmd: Commands,
    mut expired_q: Query<(Entity, &mut CmpTimeToLife)>,
    time: Res<Time>,
) {
    for (ent, mut cmp) in &mut expired_q {
        if cmp.seconds_left > 0.0 {
            cmp.seconds_left -= time.delta().as_secs_f32();
            continue;
        }

        cmd.entity(ent).despawn_recursive();
    }
}

pub fn update_mouse_pos_resource(
    mut res: ResMut<ResMouse>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CmpMainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    let screen_pos = match window.cursor_position() {
        Some(window_pos) => V2::from_2d(window_pos),
        None => return,
    };

    let world_pos = match camera.viewport_to_world(camera_transform, screen_pos.as_2d()) {
        Ok(world_pos) => V2::from_2d(world_pos.origin.truncate()),
        Err(_) => return,
    };

    res.world_pos = world_pos;
    res.screen_pos = screen_pos;
}
