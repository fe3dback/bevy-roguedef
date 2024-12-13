use bevy::prelude::{Query, With};

use crate::components::transform::CmpTransform2D;
use crate::components::unit_creature::CmpUnitBuilding;
use crate::game::teams::CmpTeam;
use crate::game::weapons::CmpWeapon;

// shitty n^ algo
pub fn tower_auto_attack_nearest_enemies(
    mut towers: Query<(&mut CmpWeapon, &CmpTeam, &CmpTransform2D), With<CmpUnitBuilding>>,
    possible_targets: Query<(&CmpTransform2D, &CmpTeam)>,
) {
    for (mut tower, tower_team, tower_trm) in &mut towers {
        let mut found = false;

        for (target_trm, target_team) in &possible_targets {
            if found {
                continue;
            }

            if !tower_team.team.is_enemy_with(target_team.team) {
                continue;
            }

            if tower_trm.position.distance(target_trm.position) > 35.0 {
                continue;
            }

            tower.aim_world_pos = target_trm.position;
            tower.trigger_press();
            found = true;
        }
    }
}
