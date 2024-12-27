use bevy::prelude::{Query, Without};
use brg_core::prelude::CmpTransform2D;

use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
use crate::game::damage::DamageKind;
use crate::game::teams::CmpTeam;
use crate::game::weapons::CmpWeapon;

// shitty n^ algo
pub fn ai_auto_attack_nearest_enemies(
    mut towers: Query<(&mut CmpWeapon, &CmpTeam, &CmpTransform2D), Without<CmpUnitCreaturePlayer>>,
    possible_targets: Query<(&CmpTransform2D, &CmpTeam)>,
) {
    for (mut ai_creature, ai_team, ai_trm) in &mut towers {
        let mut found = false;

        for (target_trm, target_team) in &possible_targets {
            if found {
                continue;
            }

            if !ai_team.team.is_enemy_with(target_team.team) {
                continue;
            }

            let max_distance = match ai_creature.current.damage.kind {
                DamageKind::Melee => 5.0,
                DamageKind::RangedSimple => 35.0,
                DamageKind::Fire => 40.0,
                _ => 10.0,
            };

            if ai_trm.position.distance(target_trm.position) > max_distance {
                continue;
            }

            ai_creature.aim_world_pos = target_trm.position;
            ai_creature.trigger_press();
            found = true;
        }
    }
}
