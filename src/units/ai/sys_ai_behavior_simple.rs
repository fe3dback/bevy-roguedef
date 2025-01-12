use std::time::Duration;

use bevy::prelude::{Entity, Query, Res, Time, With};
use brg_core::prelude::V2;
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp_ai::{CmpAiBehaviorSimple, CmpAiControllable};
use crate::units::cmp_team::CmpTeam;
use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::stats::health::cmp_health::CmpHealth;

pub fn ai_simple_move_to_enemy(
    mut q_agents: Query<
        (
            &CmpTransform2D,
            &CmpTeam,
            &mut CmpAiBehaviorSimple,
            &mut CmpUnitMovementInput,
        ),
        With<CmpAiControllable>,
    >,
    q_targets: Query<(Entity, &CmpTransform2D, &CmpTeam, &CmpHealth)>,
    time: Res<Time>,
) {
    for (trm, team, mut behavior, mut input) in &mut q_agents {
        // reset target if dead
        if let Some(target_ent) = behavior.last_target {
            match q_targets.get(target_ent) {
                Err(_) => behavior.last_target = None,
                Ok((_, target_trm, _, target_health)) => match target_health.is_alive() {
                    true => behavior.last_known_enemy_pos = target_trm.position,
                    false => behavior.last_target = None,
                },
            }
        }

        // track target
        behavior.since_last_scan += time.delta();
        if behavior.last_target.is_none()
            && behavior.since_last_scan >= Duration::from_secs_f32(1.0)
        {
            behavior.since_last_scan = Duration::ZERO;

            for (target_ent, target_trm, target_team, target_health) in &q_targets {
                if team.team.is_friendly_with(target_team.team) {
                    continue;
                }

                if !target_health.is_alive() {
                    continue;
                }

                if target_health.is_invulnerable() {
                    continue;
                }

                let distance = trm.position.distance(target_trm.position);
                if distance > behavior.agro_distance {
                    continue;
                }

                behavior.last_target = Some(target_ent);
                behavior.last_known_enemy_pos = target_trm.position;
            }
        }

        // reset input
        input.direction_vector = V2::ZERO;

        // set input in all ok
        if !behavior.last_target.is_none() {
            let distance = trm.position.distance(behavior.last_known_enemy_pos);
            if distance > behavior.agro_distance * 1.5 {
                continue;
            }

            if distance < 0.5 {
                continue;
            }

            input.direction_vector = trm.position.as_norm_dir_to(behavior.last_known_enemy_pos);
        }
    }
}
