use std::time::Duration;

use bevy::prelude::{warn, Asset, Assets, Entity, EventWriter, Query, Res, Time};
use brg_scene::prelude::AssetWeapon;

use crate::units::weapon::cmp_weapon::CmpWeaponHolder;
use crate::units::weapon::enum_fire_trigger::EWeaponTrigger;
use crate::units::weapon::enum_phase::EShootingPhase;
use crate::units::weapon::evt_reload::EvtWeaponReload;
use crate::units::weapon::evt_shot::EvtWeaponShot;

pub fn reload(
    mut query: Query<(Entity, &mut CmpWeaponHolder)>,
    time: Res<Time>,
    weapons: Res<Assets<AssetWeapon>>,
    mut event_reload_writer: EventWriter<EvtWeaponReload>,
) {
    for (weapon_owner, mut holder) in &mut query {
        for (_, weapon) in &mut holder.weapons {
            let Some(data) = weapons.get(&weapon.handle) else {
                warn!("invalid weapon handle {:?}", weapon.handle);
                continue;
            };

            if weapon.phase != EShootingPhase::Reloading {
                // if we not have ammo -> go to reload
                if weapon.ammo_left <= 0 {
                    weapon.phase = EShootingPhase::Reloading;
                    weapon.reloading_time_left =
                        Duration::from_secs_f32(data.magazine_reload_time_sec);

                    event_reload_writer.send(EvtWeaponReload {
                        owner:  weapon_owner,
                        weapon: weapon.handle.clone(),
                    });
                    continue;
                }

                continue;
            }

            let reload_time_left = match weapon.reloading_time_left <= time.delta() {
                true => Duration::ZERO,
                false => weapon.reloading_time_left - time.delta(),
            };

            if reload_time_left <= Duration::ZERO {
                weapon.reloading_time_left = Duration::ZERO;
                weapon.phase = EShootingPhase::StandBy;
                weapon.fire_time = Duration::ZERO;
                weapon.ammo_left = data.magazine_capacity;
                continue;
            }

            weapon.reloading_time_left = reload_time_left;
        }
    }
}

pub fn release_fire_trigger(mut query: Query<&mut CmpWeaponHolder>) {
    for mut holder in &mut query {
        for (_, weapon) in &mut holder.weapons {
            if weapon.trigger.mode != EWeaponTrigger::JustReleased {
                continue;
            }

            if weapon.phase == EShootingPhase::Shooting {
                weapon.phase = EShootingPhase::StandBy;
            }
        }
    }
}

pub fn shot(
    mut query: Query<(Entity, &mut CmpWeaponHolder)>,
    time: Res<Time>,
    weapons: Res<Assets<AssetWeapon>>,
    mut event_shot_writer: EventWriter<EvtWeaponShot>,
) {
    for (weapon_owner, mut holder) in &mut query {
        for (_, weapon) in &mut holder.weapons {
            if weapon.phase == EShootingPhase::Reloading {
                continue;
            }

            if !(weapon.trigger.mode == EWeaponTrigger::Pressed
                || weapon.trigger.mode == EWeaponTrigger::JustPressed)
            {
                continue;
            }

            // start shooting
            match weapon.phase {
                EShootingPhase::Reloading => {}
                EShootingPhase::StandBy => {
                    weapon.phase = EShootingPhase::Shooting;
                }
                EShootingPhase::Shooting => {
                    weapon.fire_time = weapon.fire_time + time.delta();
                }
            };

            let Some(data) = weapons.get(&weapon.handle) else {
                warn!("invalid weapon handle {:?}", weapon.handle);
                continue;
            };

            // calculate how much ammo we can fire
            let ammo_need_fire = {
                let ammo_already_fired = data.magazine_capacity as i32 - weapon.ammo_left as i32;
                let ammo_can_be_fired =
                    f32::floor(weapon.fire_time.as_secs_f32() / data.hit_reload_time_sec) as i32;

                ammo_can_be_fired - ammo_already_fired
            };

            // wait for ammo is ready
            if ammo_need_fire <= 0 {
                continue;
            }

            // spawn projectiles
            for _ in 0..ammo_need_fire {
                if weapon.ammo_left > 0 {
                    weapon.ammo_left -= 1;
                }

                event_shot_writer.send(EvtWeaponShot {
                    owner:  weapon_owner,
                    weapon: weapon.handle.clone(),
                    aim_to: weapon.trigger.aim_world_pos,
                });
            }
        }
    }
}
