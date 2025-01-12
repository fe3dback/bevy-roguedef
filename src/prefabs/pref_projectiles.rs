use std::time::Duration;

use bevy::prelude::{default, Entity, EntityCommands, Handle, Name, StateScoped};
use brg_core::prelude::types::{Angle, Speed};
use brg_core::prelude::V2;
use brg_fundamental::prelude::{CmpCollisionVolume, CmpTimeToLife, CmpTransform2D};
use brg_scene::prelude::{AssetProjectile, InGame};

use super::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::{CmpTeam, ETeam};
use crate::units::weapon::projectiles::cmp_projectile::CmpProjectile;

pub struct ProjectileSettings {
    pub caster:   Entity,
    pub team:     ETeam,
    pub handle:   Handle<AssetProjectile>,
    pub position: V2,
    pub angle:    Angle,
}

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn projectile(&mut self, settings: &ProjectileSettings) -> EntityCommands {
        let def = &AssetProjectile::default();
        let projectile = self.assets_projectiles.get(&settings.handle).unwrap_or(def);

        self.cmd.spawn((
            StateScoped(InGame),
            Name::from(format!("prj ({:?})", settings.handle.path())),
            CmpTransform2D {
                position: settings.position,
                angle: settings.angle,
                height: 1.2,
                ..default()
            },
            CmpTeam::new(settings.team),
            CmpProjectile {
                caster:         settings.caster,
                speed:          Speed::MPS(projectile.speed_start_mps),
                acceleration:   Speed::MPS(projectile.speed_acceleration_mps),
                friendly_fire:  projectile.friendly_fire,
                hit_spell_cast: projectile.cast.handle.clone(),
                hit_sound:      projectile
                    .collide_sound
                    .clone()
                    .map(|x| x.handle.clone())
                    .or(None),
            },
            CmpCollisionVolume::Circle(0.1),
            CmpTimeToLife {
                left: Duration::from_secs_f32(projectile.life_time_sec),
            },
        ))
    }
}
