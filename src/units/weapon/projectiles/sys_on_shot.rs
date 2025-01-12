use bevy::prelude::{warn, Assets, Commands, Entity, EventReader, Query, Res, StateScoped};
use brg_fundamental::prelude::CmpTransform2D;
use brg_scene::prelude::{AssetWeapon, InGame};

use crate::prefabs::prelude::ProjectileSettings;
use crate::prefabs::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::CmpTeam;
use crate::units::weapon::evt_shot::EvtWeaponShot;

pub fn on_shot_spawn_projectile(
    mut consumer: EventReader<EvtWeaponShot>,
    mut prefabs: SupPrefabs,
    query_owner: Query<(Entity, &CmpTransform2D, &CmpTeam)>,
    weapons: Res<Assets<AssetWeapon>>,
) {
    for evt in consumer.read() {
        let Ok((owner_ent, owner_trm, owner_team)) = query_owner.get(evt.owner) else {
            warn!("skip spawn projectile: not found owner unit");
            continue;
        };

        let Some(weapon) = weapons.get(&evt.weapon) else {
            warn!("skip spawn projectile: not found weapon by handle");
            continue;
        };

        prefabs.projectile(&ProjectileSettings {
            caster:   owner_ent,
            team:     owner_team.team,
            handle:   weapon.projectile.handle.clone(),
            position: owner_trm.position,
            angle:    owner_trm.position.angle_to(evt.aim_to),
        });
    }
}
