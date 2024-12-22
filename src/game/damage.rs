use bevy::color::palettes::tailwind::{GRAY_300, LIME_800};
use bevy::prelude::{
    info,
    warn,
    Changed,
    Commands,
    Component,
    DespawnRecursiveExt,
    Entity,
    Event,
    EventReader,
    Gizmos,
    Isometry2d,
    Query,
    Reflect,
    ResMut,
    Rot2,
    Vec2,
    Vec3,
    Vec4,
};
use bevy_vector_shapes::prelude::{Alignment, RectPainter, ShapePainter};
use rand_chacha::rand_core::RngCore;

use crate::components::lib::V2;
use crate::components::transform::CmpTransform2D;
use crate::components::unit::EUnitType;
use crate::game::common::ResRandomSource;
use crate::game::sound::SupSounds;
use crate::game::teams::Team;

#[derive(Component, Reflect)]
pub struct CmpHealth {
    pub health:     f32,
    pub max_health: f32,
}

impl Default for CmpHealth {
    fn default() -> Self {
        Self {
            health:     100.0,
            max_health: 100.0,
        }
    }
}

#[derive(Reflect, Debug, Default, Copy, Clone)]
pub enum DamageKind {
    #[default]
    Melee,
    RangedSimple,
    Fire,
}

#[derive(Reflect, Debug, Copy, Clone)]
pub struct Damage {
    pub kind:         DamageKind,
    pub amount:       f32,
    pub dices_amount: u8, // how many dices we throw
    pub dice_faces:   u8, // how many faces in each dice (dmg = amount + (rnd(0,d_amount) * rnd(0,d_faces)))
}

impl Default for Damage {
    fn default() -> Self {
        Self {
            kind:         DamageKind::default(),
            amount:       1.0,
            dices_amount: 0,
            dice_faces:   0,
        }
    }
}

#[derive(Reflect, Debug, Copy, Clone)]
pub struct DamageCastSource {
    pub damage:     Damage,
    pub origin:     V2,
    pub caster:     Option<Entity>,
    pub projectile: Option<Entity>,
    pub owner_team: Team,
}

#[derive(Reflect, Debug)]
pub struct DamageCastTarget {
    pub targets: Vec<Entity>,
}

#[derive(Event)]
pub struct EvtOnDamageCast {
    pub cast:   DamageCastSource,
    pub target: DamageCastTarget,
}

pub fn damage_event_listener(
    mut consumer: EventReader<EvtOnDamageCast>,
    mut target_query: Query<(&mut CmpHealth, &CmpTransform2D)>,
    mut rand: ResMut<ResRandomSource>,
    mut sounds: SupSounds,
) {
    for evt in &mut consumer.read() {
        for t in &evt.target.targets {
            let (mut cmp, target_trm) = match target_query.get_mut(*t) {
                Ok(cmp) => cmp,
                Err(_) => continue,
            };

            let mut dmg = evt.cast.damage.amount;

            let dices = rand.rnd.next_u32() % (1 + evt.cast.damage.dices_amount as u32);
            for _ in 0..dices {
                let dice_result = rand.rnd.next_u32() % (1 + evt.cast.damage.dice_faces as u32);
                dmg += dice_result as f32;
            }

            cmp.health -= dmg;

            if dmg > 0.0 {
                sounds.play_impact(target_trm.position);
            }
        }
    }
}

pub fn death_by_health(
    mut cmd: Commands,
    entities_q: Query<(Entity, &CmpHealth, &EUnitType), Changed<CmpHealth>>,
) {
    for (entity, cmp, unittype) in entities_q.iter() {
        if cmp.health > 0.0 {
            continue;
        }

        warn!("entity died, despawning {}..", entity);
        if !unittype.is_building() {
            // todo: remove buildings from graph on destroy
            // todo: cmp events (OnAdded, OnRemoved)
            cmd.entity(entity).despawn_recursive();
        }
    }
}

// todo: replace lib to Gizmos
pub fn draw_health_bar(mut painter: ShapePainter, creatures: Query<(&CmpTransform2D, &CmpHealth)>) {
    painter.hollow = false;
    painter.corner_radii = Vec4::splat(0.2);

    for (trm, health) in &creatures {
        if health.health == health.max_health {
            continue;
        }

        let percent = (health.health / health.max_health) * 100.0;

        painter.color = GRAY_300.into();
        painter.transform.translation = trm.position.as_3d_ui() - Vec3::new(0.0, 36.0, 0.0);
        painter.rect(Vec2::new(100.0 * 0.5, 4.0));

        painter.color = LIME_800.into();
        painter.rect(Vec2::new(percent * 0.5, 3.0));
    }
}
