use std::time::Duration;

use bevy::prelude::{default, Handle, Name};
use brg_core::prelude::types::Speed;
use brg_fundamental::prelude::{CmpCollisionVolume, CmpTimeToLife, CmpTransform2D};
use brg_scene::prelude::AssetProjectile;

use super::sup_prefabs::SupPrefabs;
use crate::units::weapon::projectiles::cmp_projectile::CmpProjectile;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn projectile(
        &mut self,
        handle: Handle<AssetProjectile>,
    ) -> (
        CmpTransform2D,
        Name,
        CmpProjectile,
        CmpCollisionVolume,
        CmpTimeToLife,
    ) {
        let def = &AssetProjectile::default();
        let projectile = self.assets_projectiles.get(&handle).unwrap_or(def);

        (
            CmpTransform2D {
                height: 1.2,
                ..default()
            },
            Name::from(format!("Projectile ({:?})", handle.path())),
            CmpProjectile {
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
        )
    }
}
