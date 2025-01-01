use bevy::prelude::{
    info,
    Added,
    Children,
    Commands,
    Entity,
    HierarchyQueryExt,
    Query,
    Res,
    ResMut,
    StateScoped,
};
use bevy::scene::SceneInstance;
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::{Range, V3};
use brg_fundamental::prelude::{CmpTerrainMarkerMesh, ResHeightmap, SupWorldRayCast};
use brg_scene::prelude::InGame;

use crate::prefabs::sup_prefabs::SupPrefabs;

pub fn spawn_example_objects(mut commands: Commands, mut prefabs: SupPrefabs) {
    let (floor, cube) = prefabs.example_scene();
    commands.spawn((floor, StateScoped(InGame)));
    commands.spawn((cube, StateScoped(InGame)));

    commands.spawn((prefabs.example_terrain(), StateScoped(InGame)));
}

pub fn debug_mark_terrain_as_heightmap_source(
    mut commands: Commands,
    q: Query<Entity, Added<SceneInstance>>,
    children: Query<&Children>,
) {
    for parent in &q {
        for child in children.iter_descendants(parent) {
            commands.entity(child).insert(CmpTerrainMarkerMesh);
        }
    }
}

pub fn debug_tmp_update_heightmap_from_terrain(
    mut rcast: SupWorldRayCast,
    mut hm: ResMut<ResHeightmap>,
    time: Res<bevy::time::Time>,
) {
    if time.elapsed_secs() < 1.0 || time.elapsed_secs() > 1.5 {
        return;
    }

    const RAY_MIN: f32 = -100.0;
    const RAY_MAX: f32 = 100.0;

    let range = Range::new(-20, -20, 20, 20);
    let mut points: Vec<f32> = vec![];

    for tile in &range {
        let pos = tile.position();
        let floor = V3::new(pos.x, pos.y, RAY_MIN);
        let ceil = V3::new(pos.x, pos.y, RAY_MAX);

        let abs_height = rcast.ray_cast(ceil, floor).h;
        let height = abs_height / TERRAIN_HEIGHT;

        hm.tiles.insert(tile, height);
        points.push(height);
    }

    info!(
        "points=[{}]",
        points
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
