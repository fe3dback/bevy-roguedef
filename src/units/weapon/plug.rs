use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::GameSystemSet;

use crate::units::weapon::cmp_weapon::CmpWeaponHolder;
use crate::units::weapon::evt_reload::EvtWeaponReload;
use crate::units::weapon::evt_shot::EvtWeaponShot;
use crate::units::weapon::projectiles;
use crate::units::weapon::sys_handle_events::{on_reload, on_shot};
use crate::units::weapon::sys_reset_trigger::auto_reset_weapon_trigger;
use crate::units::weapon::sys_weapons::{release_fire_trigger, reload, shot};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(projectiles::plug::Plug)
            //
            .register_type::<CmpWeaponHolder>()
            .add_event::<EvtWeaponReload>()
            .add_event::<EvtWeaponShot>()
            //
            .add_systems(Update, (
                auto_reset_weapon_trigger,
                reload,
                release_fire_trigger,
                shot,
                on_reload,
                on_shot,
            ).in_set(GameSystemSet::InGamePrepareWeapons))
        //-
        ;
    }
}
