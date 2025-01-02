use std::time::Duration;

use bevy::prelude::{Component, Handle, Reflect};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::V2;
use brg_scene::prelude::AssetWeapon;

use crate::units::weapon::enum_fire_trigger::EWeaponTrigger;
use crate::units::weapon::enum_phase::EShootingPhase;

pub type WeaponID = String;

#[derive(Component, Default, Debug, Reflect)]
pub struct CmpWeaponHolder {
    pub weapons: HashMap<WeaponID, Weapon>,
}

#[derive(Debug, Reflect)]
pub struct Weapon {
    pub handle:              Handle<AssetWeapon>,
    pub phase:               EShootingPhase,
    pub ammo_left:           u32,
    pub reloading_time_left: Duration,
    pub fire_time:           Duration,
    pub trigger:             WeaponFireTrigger,
}

#[derive(Debug, Reflect)]
pub struct WeaponFireTrigger {
    pub(super) aim_world_pos: V2,
    pub(super) mode:          EWeaponTrigger,
    pub(super) last_pressed:  bool,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            handle:              Handle::default(),
            phase:               EShootingPhase::Reloading,
            ammo_left:           0,
            reloading_time_left: Duration::ZERO,
            fire_time:           Duration::ZERO,
            trigger:             WeaponFireTrigger {
                aim_world_pos: V2::ZERO,
                mode:          EWeaponTrigger::Released,
                last_pressed:  false,
            },
        }
    }
}

impl WeaponFireTrigger {
    pub fn aim_to(&mut self, target: V2) {
        self.aim_world_pos = target;
    }

    pub fn press(&mut self) {
        self.last_pressed = true;
        self.mode = match self.mode {
            EWeaponTrigger::Released => EWeaponTrigger::JustPressed,
            EWeaponTrigger::JustPressed => EWeaponTrigger::Pressed,
            EWeaponTrigger::Pressed => EWeaponTrigger::Pressed,
            EWeaponTrigger::JustReleased => EWeaponTrigger::JustPressed,
        }
    }

    pub fn release(&mut self) {
        self.last_pressed = false;
        self.mode = match self.mode {
            EWeaponTrigger::Released => EWeaponTrigger::Released,
            EWeaponTrigger::JustReleased => EWeaponTrigger::Released,
            EWeaponTrigger::Pressed => EWeaponTrigger::JustReleased,
            EWeaponTrigger::JustPressed => EWeaponTrigger::JustReleased,
        }
    }
}
