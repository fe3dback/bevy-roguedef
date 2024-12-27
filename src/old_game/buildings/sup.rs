use std::time::Duration;

use bevy::asset::AssetServer;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{AlphaMode, Commands, Name, Rectangle, Res, ResMut};
use bevy::utils::default;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dBundle, Sprite3dParams};
use brg_core::prelude::{select_n_tiles_around_position, CmpTransform2D, Tile, V2};

use super::electro::enums::EArchetype;
use super::electro::res_graph::ResBuildingWorldGraphs;
use crate::components::unit::EUnitType;
use crate::game::buildings::electro::cmp::{CmpBuildingElectricity, CmpBuildingOccupied};
use crate::game::buildings::electro::cmp_debug::CmpDebugElectricityOutline;
use crate::game::buildings::electro::dto::ChannelState;
use crate::game::buildings::electro::types::MAX_CHANNELS;
use crate::game::collisions::CmpCollisionDesiredVolume;
use crate::game::damage::{CmpHealth, Damage, DamageKind};
use crate::game::teams::{CmpTeam, Team};
use crate::game::weapons::{CmpWeapon, Weapon};
use crate::plugins::assets::asset::GameAssets;

#[derive(SystemParam)]
pub struct SupBuildingSpawner<'w, 's> {
    cmd:           Commands<'w, 's>,
    assets:        Res<'w, GameAssets>,
    asset_server:  Res<'w, AssetServer>,
    graph:         ResMut<'w, ResBuildingWorldGraphs>,
    sprite_params: Sprite3dParams<'w, 's>,
}

impl<'w, 's> SupBuildingSpawner<'w, 's> {
    pub fn spawn_pole(&mut self, at: V2) {
        let building = self.create_building(at, "pole", 20, 1);
        let center = building.2.position;

        let id = self
            .cmd
            .spawn((
                building,
                CmpBuildingElectricity {
                    is_source:                  false,
                    can_consume:                true,
                    can_produce:                true,
                    capacity:                   2.5,
                    count_channels:             1,
                    channels:                   self.create_channels(0.0, 2.5, 2.5, 2.5),
                    throughput_max_in:          2.5,
                    throughput_max_out:         2.5,
                    connection_radius_in_tiles: 6.0,
                },
            ))
            .id();

        self.graph.insert(id, center, EArchetype::Pole, true, true);
    }

    pub fn spawn_tower(&mut self, at: V2) {
        let building = self.create_building(at, "tower", 65, 2);
        let center = building.2.position;

        let id = self
            .cmd
            .spawn((
                building,
                CmpBuildingElectricity {
                    is_source:                  false,
                    can_consume:                true,
                    can_produce:                false,
                    capacity:                   10.0,
                    count_channels:             1,
                    channels:                   self.create_channels(0.0, 10.0, 5.0, 0.0),
                    throughput_max_in:          5.0,
                    throughput_max_out:         0.0,
                    connection_radius_in_tiles: 3.0,
                },
                CmpWeapon {
                    current: Weapon {
                        name:                 String::from("tower-fastgun"),
                        ammo_in_magazine:     100,
                        damage:               Damage {
                            kind:         DamageKind::RangedSimple,
                            amount:       2.0,
                            dice_faces:   1,
                            dices_amount: 2,
                        },
                        shooting_reload_time: Duration::from_millis(150),
                        magazine_reload_time: Duration::from_millis(800),
                    },
                    ..default()
                },
            ))
            .id();

        self.graph
            .insert(id, center, EArchetype::Tower, false, true);
    }

    pub fn spawn_castle(&mut self, at: V2) {
        let building = self.create_building(at, "castle", 500, 6);
        let center = building.2.position;

        let id = self
            .cmd
            .spawn((
                building,
                CmpBuildingElectricity {
                    is_source:                  false,
                    can_consume:                true,
                    can_produce:                true,
                    capacity:                   100.0,
                    count_channels:             1,
                    channels:                   self.create_channels(50.0, 100.0, 50.0, 50.0),
                    throughput_max_in:          50.0,
                    throughput_max_out:         50.0,
                    connection_radius_in_tiles: 3.0,
                },
            ))
            .id();

        self.graph
            .insert(id, center, EArchetype::Castle, true, true);
    }

    pub fn spawn_source(&mut self, at: V2) {
        let mut building = self.create_building(at, "source", 10000, 8);
        let center = building.2.position;

        building.3.team = Team::Neutral;
        let id = self
            .cmd
            .spawn((
                building,
                CmpBuildingElectricity {
                    is_source:                  true,
                    can_consume:                false,
                    can_produce:                true,
                    capacity:                   1000.0,
                    count_channels:             1,
                    channels:                   self.create_channels(1000.0, 1000.0, 0.0, 100.0),
                    throughput_max_in:          0.0,
                    throughput_max_out:         100.0,
                    connection_radius_in_tiles: 2.0,
                },
            ))
            .id();

        self.graph
            .insert(id, center, EArchetype::Source, true, false);
    }

    fn create_channels(
        &self,
        charge: f32,
        cap: f32,
        thro_in: f32,
        thro_out: f32,
    ) -> [ChannelState; 16] {
        // setup channels and use first as default
        let mut channels: [ChannelState; MAX_CHANNELS] = [ChannelState::default(); MAX_CHANNELS];

        // todo: auto channels
        channels[0] = ChannelState {
            charge,
            capacity: cap,
            throughput_max_in: thro_in,
            throughput_max_out: thro_out,
            throughput_in: 0.0,
            throughput_out: 0.0,
        };

        channels
    }

    fn create_building(
        &mut self,
        at: V2,
        name: &str,
        health: u32,
        tiles: u8,
    ) -> (
        Name,
        EUnitType,
        CmpTransform2D,
        CmpTeam,
        CmpHealth,
        CmpBuildingOccupied,
        CmpDebugElectricityOutline,
        CmpCollisionDesiredVolume,
        Sprite3dBundle,
    ) {
        let range = select_n_tiles_around_position(at, tiles as i32, tiles as i32);
        let pos = range.position_center();

        (
            Name::from(name),
            EUnitType::Building,
            CmpTransform2D {
                position: pos,
                angle: 0.0,
                ..default()
            },
            CmpTeam { team: Team::Player },
            CmpHealth {
                health:     health as f32,
                max_health: health as f32,
            },
            CmpBuildingOccupied {
                grid_width:    tiles,
                grid_height:   tiles,
                grid_position: Tile::at(range.min_x, range.min_y),
            },
            CmpDebugElectricityOutline::default(),
            CmpCollisionDesiredVolume::Aabb(Rectangle::new(tiles as f32, tiles as f32)),
            Sprite3dBuilder {
                image: self
                    .assets
                    .sprites
                    .get(format!("sprites/buildings/{}.png", name).as_str())
                    .unwrap()
                    .clone_weak(),
                pixels_per_metre: 24.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                double_sided: true,
                ..default()
            }
            .bundle(&mut self.sprite_params),
        )
    }
}
