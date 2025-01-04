use bevy::prelude::{warn, Assets, Commands, EventReader, Query, Res};
use brg_fundamental::prelude::CmpTransform2D;
use brg_scene::prelude::AssetWeapon;

use crate::prefabs::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::CmpTeam;
use crate::units::weapon::evt_shot::EvtWeaponShot;

pub fn on_shot_spawn_projectile(
    mut cmd: Commands,
    mut consumer: EventReader<EvtWeaponShot>,
    mut prefabs: SupPrefabs,
    query_owner: Query<(&CmpTransform2D, &CmpTeam)>,
    weapons: Res<Assets<AssetWeapon>>,
) {
    for evt in consumer.read() {
        let Ok((owner_trm, owner_team)) = query_owner.get(evt.owner) else {
            warn!("skip spawn projectile: not found owner unit");
            continue;
        };

        let Some(weapon) = weapons.get(&evt.weapon) else {
            warn!("skip spawn projectile: not found weapon by handle");
            continue;
        };

        let mut projectile = prefabs.projectile(weapon.projectile.handle.clone());
        projectile.0.position = owner_trm.position;
        projectile.0.angle = owner_trm.position.angle_to(evt.aim_to);

        cmd.spawn((projectile, CmpTeam::new(owner_team.team)));
    }
}
