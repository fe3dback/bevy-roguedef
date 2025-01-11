use bevy::ecs::system::SystemParam;
use bevy::prelude::{Assets, Commands, Entity, Handle, Query, Res, ResMut};
use brg_core::prelude::ResRandomSource;
use brg_fundamental::common::enum_randomizer_kind::RandomizerKindSpells;
use brg_scene::prelude::{AssetEffect, AssetEffectDamage, AssetSpell};

use crate::units::cmp_team::CmpTeam;
use crate::units::stats::health::cmp_health::CmpHealth;

#[derive(SystemParam)]
pub struct SupSpells<'w, 's> {
    pub(super) cmd:                 Commands<'w, 's>,
    pub(super) asset_spells:        Res<'w, Assets<AssetSpell>>,
    pub(super) asset_effects:       Res<'w, Assets<AssetEffect>>,
    pub(super) rand:                ResMut<'w, ResRandomSource<RandomizerKindSpells>>,
    pub(super) query_caster_team:   Query<'w, 's, &'static CmpTeam>,
    pub(super) query_damage_target: Query<'w, 's, (&'static CmpTeam, &'static mut CmpHealth)>,
}

impl<'w, 's> SupSpells<'w, 's> {
    pub fn cast_target_spell(
        &mut self,
        caster: Entity,
        target: Entity,
        spell_handle: Handle<AssetSpell>,
    ) {
        let Some(spell) = self.asset_spells.get(&spell_handle) else {
            return;
        };

        self.apply_onetime_effect(caster, target, spell.apply_one_time.handle.clone());
    }

    fn apply_onetime_effect(
        &mut self,
        caster: Entity,
        target: Entity,
        effect_handle: Handle<AssetEffect>,
    ) {
        let Some(effect) = self.asset_effects.get(&effect_handle) else {
            return;
        };

        if let Some(damage) = effect.damage {
            self.apply_damage_effect(caster, target, damage);
        }
    }

    fn apply_damage_effect(&mut self, caster: Entity, target: Entity, damage: AssetEffectDamage) {
        let amount = damage.base
            + self
                .rand
                .rand_roll_dices(damage.dice_count, damage.dice_faces);

        let Ok((target_team, mut target_health)) = self.query_damage_target.get_mut(target) else {
            return;
        };

        let Ok(caster_team) = self.query_caster_team.get(caster) else {
            return;
        };

        if caster_team.team.is_friendly_with(target_team.team) && !damage.allow_friendly_fire {
            return;
        }

        target_health.take_damage(amount as f32);
    }
}
