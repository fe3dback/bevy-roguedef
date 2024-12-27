use std::time::Duration;

use bevy::core::Name;
use bevy::prelude::{
    default,
    info,
    Bundle,
    ButtonInput,
    Circle,
    Commands,
    Component,
    Entity,
    MouseButton,
    Query,
    Reflect,
    ReflectComponent,
    ReflectResource,
    Res,
    ResMut,
    Resource,
    Time,
    With,
};
use bevy_trait_query::One;
use brg_core::prelude::{CmpTransform2D, V2};

use crate::components::unit::EUnitType;
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
use crate::game::buildings::electro::cmp::CmpBuildingElectricity;
use crate::game::collisions::CmpCollisionDesiredVolume;
use crate::game::common::{CmpTimeToLife, ResMouse};
use crate::game::damage::Damage;
use crate::game::energy::CmpEnergyContainer;
use crate::game::projectiles::CmpProjectile;
use crate::game::sound::SupSounds;
use crate::game::teams::{CmpTeam, Team};

#[derive(Reflect)]
pub struct Weapon {
    pub name:                 String,
    pub damage:               Damage,
    pub shooting_reload_time: Duration,
    pub ammo_in_magazine:     u8,
    pub magazine_reload_time: Duration,
}

#[derive(Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ShootingPhase {
    StandBy,
    Shooting,
    Reloading,
}

#[derive(Component, Reflect)]
pub struct CmpWeapon {
    pub current:             Weapon,
    pub phase:               ShootingPhase,
    pub ammo_left:           u8,
    pub reloading_time_left: Duration,
    pub fire_time:           Duration,
    pub aim_world_pos:       V2,
    pub trigger:             EWeaponTrigger,
    pub last_pressed:        bool,
}

#[derive(Reflect, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EWeaponTrigger {
    #[default]
    Released,
    JustPressed,
    Pressed,
    JustReleased,
}

impl CmpWeapon {
    pub fn trigger_press(&mut self) {
        self.last_pressed = true;
        self.trigger = match self.trigger {
            EWeaponTrigger::Released => EWeaponTrigger::JustPressed,
            EWeaponTrigger::JustPressed => EWeaponTrigger::Pressed,
            EWeaponTrigger::Pressed => EWeaponTrigger::Pressed,
            EWeaponTrigger::JustReleased => EWeaponTrigger::JustPressed,
        }
    }

    pub fn trigger_release(&mut self) {
        self.last_pressed = false;
        self.trigger = match self.trigger {
            EWeaponTrigger::Released => EWeaponTrigger::Released,
            EWeaponTrigger::JustReleased => EWeaponTrigger::Released,
            EWeaponTrigger::Pressed => EWeaponTrigger::JustReleased,
            EWeaponTrigger::JustPressed => EWeaponTrigger::JustReleased,
        }
    }
}

impl Default for CmpWeapon {
    fn default() -> Self {
        Self {
            current:             Weapon {
                name:                 String::from("ak-magic-47"),
                damage:               Damage {
                    amount: 11.0,
                    dice_faces: 2,
                    dices_amount: 3,
                    ..default()
                },
                ammo_in_magazine:     30,
                shooting_reload_time: Duration::from_millis(80),
                magazine_reload_time: Duration::from_millis(1800),
            },
            phase:               ShootingPhase::Reloading,
            ammo_left:           0,
            reloading_time_left: Duration::ZERO,
            fire_time:           Duration::ZERO,
            aim_world_pos:       V2::ZERO,
            trigger:             EWeaponTrigger::Released,
            last_pressed:        false,
        }
    }
}

pub fn auto_reset_weapon_trigger(mut query: Query<&mut CmpWeapon>) {
    for mut weapon in &mut query {
        if !weapon.last_pressed {
            weapon.trigger_release();
            continue;
        }

        weapon.last_pressed = false;
    }
}

pub fn player_trigger_shot(
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<ResMouse>,
    mut player_query: Query<(&mut CmpWeapon), With<CmpUnitCreaturePlayer>>,
) {
    for (mut weapon) in &mut player_query {
        if mouse.pressed(MouseButton::Left) {
            weapon.aim_world_pos = mouse_data.world_pos;
            weapon.trigger_press();
        }
    }
}

pub fn shooting(
    mut cmd: Commands,
    mut sounds: SupSounds,
    mut query: Query<(
        Entity,
        &mut CmpWeapon,
        &CmpTeam,
        &CmpTransform2D,
        One<&mut dyn CmpEnergyContainer>,
        &EUnitType,
    )>,
    time: Res<Time>,
) {
    for (ent, mut weapon, team, trm2d, mut energy, utype) in &mut query {
        if weapon.phase == ShootingPhase::Reloading {
            let reload_time_left = match weapon.reloading_time_left <= time.delta() {
                true => Duration::ZERO,
                false => weapon.reloading_time_left - time.delta(),
            };

            if reload_time_left <= Duration::ZERO {
                weapon.reloading_time_left = Duration::ZERO;
                weapon.phase = ShootingPhase::StandBy;
                weapon.fire_time = Duration::ZERO;
                weapon.ammo_left = weapon.current.ammo_in_magazine;
                continue;
            }

            weapon.reloading_time_left = reload_time_left;
        }

        if weapon.trigger == EWeaponTrigger::JustReleased {
            if weapon.phase == ShootingPhase::Shooting {
                weapon.phase = ShootingPhase::StandBy;
                continue;
            }
        }

        if !(weapon.trigger == EWeaponTrigger::Pressed
            || weapon.trigger == EWeaponTrigger::JustPressed)
        {
            continue;
        }

        // reload
        let mut shot = false;

        match weapon.phase {
            ShootingPhase::Reloading => {}
            ShootingPhase::StandBy => {
                weapon.phase = ShootingPhase::Shooting;
                shot = true;
            }
            ShootingPhase::Shooting => {
                weapon.fire_time = weapon.fire_time + time.delta();
                shot = true;
            }
        };

        if !shot {
            continue;
        }

        // if we not have ammo -> go to reload
        if weapon.ammo_left <= 0 {
            weapon.phase = ShootingPhase::Reloading;
            weapon.reloading_time_left = weapon.current.magazine_reload_time;
            sounds.play_reload(trm2d.position);
            continue;
        }

        // calculate how much ammo we can fire
        let ammo_already_fired = weapon.current.ammo_in_magazine - weapon.ammo_left;
        let ammo_can_be_fired = f32::floor(
            weapon.fire_time.as_secs_f32() / weapon.current.shooting_reload_time.as_secs_f32(),
        ) as i32;
        let mut ammo_need_fire = ammo_can_be_fired - ammo_already_fired as i32;
        if ammo_need_fire > 100 {
            ammo_need_fire = 100;
        }

        // wait for ammo is ready
        if ammo_need_fire <= 0 {
            continue;
        }

        for _ in 0..ammo_need_fire {
            if utype.is_building() && !energy.try_spend(0.05) {
                continue;
            }

            if weapon.ammo_left > 0 {
                weapon.ammo_left -= 1;
            }

            sounds.play_shot(trm2d.position);
            cmd.spawn((
                Name::from(format!("bullet {:?}", team.team)),
                CmpTimeToLife { seconds_left: 2.0 },
                CmpTransform2D {
                    position: trm2d.position,
                    angle: trm2d.position.angle_to(weapon.aim_world_pos),
                    ..default()
                },
                CmpCollisionDesiredVolume::Circle(Circle::new(0.1)),
                CmpProjectile {
                    team: team.team,
                    caster: Some(ent),
                    allow_friendly_fire: false,
                    damage: weapon.current.damage,
                    ..default()
                },
            ));
        }
    }
}
