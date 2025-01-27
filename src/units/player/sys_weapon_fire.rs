use bevy::prelude::{ButtonInput, MouseButton, Query, Res, With};
use brg_fundamental::prelude::ResCoords;

use crate::units::player::cmp_marker_player::CmpMarkerPlayer;
use crate::units::weapon::cmp_weapon::CmpWeaponHolder;

pub fn weapon_trigger_fire(
    mouse: Res<ButtonInput<MouseButton>>,
    coords: Res<ResCoords>,
    mut player_query: Query<&mut CmpWeaponHolder, With<CmpMarkerPlayer>>,
) {
    for mut holder in &mut player_query {
        for (_, weapon) in &mut holder.weapons {
            if mouse.pressed(MouseButton::Left) {
                weapon.trigger.aim_to(coords.mouse_world_pos);
                weapon.trigger.press();
            }
        }
    }
}
