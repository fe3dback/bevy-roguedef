use bevy::prelude::Query;

use crate::units::weapon::cmp_weapon::CmpWeaponHolder;

pub fn auto_reset_weapon_trigger(mut query: Query<&mut CmpWeaponHolder>) {
    for mut holder in &mut query {
        for (_, weapon) in &mut holder.weapons {
            if !weapon.trigger.last_pressed {
                weapon.trigger.release();
                continue;
            }

            weapon.trigger.last_pressed = false;
        }
    }
}
