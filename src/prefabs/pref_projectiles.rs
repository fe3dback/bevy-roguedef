use bevy::prelude::{default, Handle, Name};
use brg_core::prelude::types::Speed;
use brg_fundamental::prelude::CmpTransform2D;
use brg_scene::prelude::AssetProjectile;

use crate::prefabs::sup_prefabs::SupPrefabs;
use crate::units::weapon::projectiles::cmp_projectile::CmpProjectile;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn projectile(
        &mut self,
        handle: Handle<AssetProjectile>,
    ) -> (CmpTransform2D, Name, CmpProjectile) {
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
                hit_spell_cast: projectile.cast.handle.clone(),
            },
        )
    }
}
