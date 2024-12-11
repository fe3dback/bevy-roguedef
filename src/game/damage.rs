use {
    crate::{
        components::{lib::V2, transform::CmpTransform2D},
        game::{common::ResRandomSource, sound::SupSounds, teams::Team},
    },
    bevy::{
        color::palettes::tailwind::{GRAY_300, LIME_800},
        prelude::{
            info,
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
        },
    },
    rand_chacha::rand_core::RngCore,
};

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
    entities_q: Query<(Entity, &CmpHealth), Changed<CmpHealth>>,
) {
    for (entity, cmp) in entities_q.iter() {
        if cmp.health > 0.0 {
            continue;
        }

        cmd.entity(entity).despawn_recursive();
    }
}

pub fn draw_health_bar(mut gz: Gizmos, creatures: Query<(&CmpTransform2D, &CmpHealth)>) {
    for (trm, health) in &creatures {
        let percent = (health.health / health.max_health) * 100.0;

        gz.rect_2d(
            Isometry2d::new((trm.position - V2::new(0.0, 36.0)).as_2d(), Rot2::IDENTITY),
            V2::new(100.0 * 0.5, 4.0).as_2d(),
            GRAY_300,
        );
        gz.rect_2d(
            Isometry2d::new((trm.position - V2::new(0.0, 36.0)).as_2d(), Rot2::IDENTITY),
            V2::new(percent * 0.5, 2.0).as_2d(),
            LIME_800,
        );
    }
}
