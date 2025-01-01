use bevy::prelude::{
    Camera,
    Commands,
    Component,
    DespawnRecursiveExt,
    Entity,
    GlobalTransform,
    Query,
    Reflect,
    ReflectResource,
    Res,
    ResMut,
    Resource,
    Time,
    Window,
    With,
};
use bevy::window::PrimaryWindow;
use brg_core::prelude::V2;
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct ResRandomSource {
    pub rnd: ChaCha8Rng,
}

#[derive(Component, Reflect)]
pub struct CmpTimeToLife {
    pub seconds_left: f32,
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
