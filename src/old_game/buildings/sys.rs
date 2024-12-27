use bevy::prelude::{
    AssetServer,
    Assets,
    ButtonInput,
    Handle,
    KeyCode,
    Local,
    MouseButton,
    Res,
    ResMut,
    Resource,
};
use brg_core::prelude::Tile;

use crate::game::buildings::electro::enums::EArchetype;
use crate::game::buildings::sup::SupBuildingSpawner;
use crate::game::common::ResMouse;
use crate::plugins::assets::asset_ldtk_circuit::AssetLdtkCircuit;

#[derive(Resource, Default)]
pub struct ResLdtkHandles {
    handle: Handle<AssetLdtkCircuit>,
}

#[derive(Resource, Default)]
pub struct ResBuildingDebugChoose {
    pub choosed_building: EArchetype,
}

pub fn load_ldtk_circuit(loader: Res<AssetServer>, mut r: ResMut<ResLdtkHandles>) {
    r.handle = loader.load("data/dev/ldtk_circuits/scene/BASE1.ldtkl");
}

pub fn spawn_starting_buildings(
    mut manager: SupBuildingSpawner,
    r: Res<ResLdtkHandles>,
    ldtk_circuits: Res<Assets<AssetLdtkCircuit>>,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }

    let scene = match ldtk_circuits.get(&r.handle) {
        Some(c) => c,
        None => return,
    };

    *spawned = true;

    let scene_offset = Tile::at(
        scene.width / 2 / scene.px_per_meter,
        scene.height / 2 / scene.px_per_meter,
    );

    for (_, entity) in scene.buildings.iter().enumerate() {
        let pos = Tile::at(entity.pos.x - scene_offset.x, entity.pos.y - scene_offset.y);

        match entity.class.as_str() {
            "SOURCE" => manager.spawn_source(pos.position_center()),
            "CASTLE" => manager.spawn_tower(pos.position_center()),
            "POLE" => manager.spawn_pole(pos.position_center()),
            "TOWER" => manager.spawn_tower(pos.position_center()),
            _ => panic!("unknown ldtk entity type {}", entity.class),
        };
    }
}

pub fn choose_debug_building_to_spawn(
    kbr: Res<ButtonInput<KeyCode>>,
    mut build_menu: ResMut<ResBuildingDebugChoose>,
) {
    if kbr.just_pressed(KeyCode::Digit1) {
        build_menu.choosed_building = EArchetype::Pole;
    }
    if kbr.just_pressed(KeyCode::Digit2) {
        build_menu.choosed_building = EArchetype::Tower;
    }
    if kbr.just_pressed(KeyCode::Digit3) {
        build_menu.choosed_building = EArchetype::Castle;
    }
    if kbr.just_pressed(KeyCode::Digit4) {
        build_menu.choosed_building = EArchetype::Source;
    }
}

pub fn spawn_building_on_mouse_click(
    mut manager: SupBuildingSpawner,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<ResMouse>,
    build_menu: Res<ResBuildingDebugChoose>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        match build_menu.choosed_building {
            EArchetype::Pole => manager.spawn_pole(mouse_data.world_pos),
            EArchetype::Tower => manager.spawn_tower(mouse_data.world_pos),
            EArchetype::Castle => manager.spawn_castle(mouse_data.world_pos),
            EArchetype::Source => manager.spawn_source(mouse_data.world_pos),
        }
    }
}
