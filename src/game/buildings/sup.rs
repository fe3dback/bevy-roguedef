use std::time::Duration;

use bevy::asset::AssetServer;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Name, Rectangle, Res, Sprite};
use bevy::utils::default;

use crate::components::lib::V2;
use crate::components::tiles::select_n_tiles_around_position;
use crate::components::transform::CmpTransform2D;
use crate::consts::PIXELS_PER_METER;
use crate::game::buildings::CmpBuildingElectricity;
use crate::game::collisions::CmpCollisionDesiredVolume;
use crate::game::damage::{CmpHealth, Damage, DamageKind};
use crate::game::teams::{CmpTeam, Team};
use crate::game::weapons::{CmpWeapon, Weapon};

#[derive(SystemParam)]
pub struct SupBuildingSpawner<'w, 's> {
    cmd:          Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's> SupBuildingSpawner<'w, 's> {
    pub fn spawn_pole(&mut self, at: V2) {
        let building = self.create_building(at, "pole", 20, 1);
        self.cmd.spawn(building);
    }

    pub fn spawn_tower(&mut self, at: V2) {
        let building = self.create_building(at, "tower", 65, 2);
        self.cmd.spawn((
            building,
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
        ));
    }

    pub fn spawn_castle(&mut self, at: V2) {
        let building = self.create_building(at, "castle", 500, 6);
        self.cmd.spawn(building);
    }

    pub fn spawn_source(&mut self, at: V2) {
        let mut building = self.create_building(at, "source", 10000, 8);
        building.2.team = Team::Neutral;
        self.cmd.spawn(building);
    }

    pub fn create_building(
        &mut self,
        at: V2,
        name: &str,
        health: u32,
        tiles: u8,
    ) -> (
        Name,
        CmpTransform2D,
        CmpTeam,
        CmpHealth,
        CmpCollisionDesiredVolume,
        Sprite,
        CmpBuildingElectricity,
    ) {
        let range = select_n_tiles_around_position(at, tiles as i32, tiles as i32);

        (
            Name::from(name),
            CmpTransform2D {
                position: range.position_center(),
                angle:    0.0,
            },
            CmpTeam { team: Team::Player },
            CmpHealth {
                health:     health as f32,
                max_health: health as f32,
            },
            CmpCollisionDesiredVolume::Aabb(Rectangle::new(
                PIXELS_PER_METER * tiles as f32,
                PIXELS_PER_METER * tiles as f32,
            )),
            Sprite::from_image(
                self.asset_server
                    .load(format!("sprites/buildings/{}.png", name)),
            ),
            CmpBuildingElectricity {},
        )
    }
}
