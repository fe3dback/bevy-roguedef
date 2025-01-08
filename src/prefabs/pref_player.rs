use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Mesh3d, Name, SpatialListener};
use brg_fundamental::prelude::{CmpCollisionVolume, CmpMarkerCameraTarget, CmpTransform2D};

use super::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::{CmpTeam, ETeam};
use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::mobs::enum_mob_type::MobKind;
use crate::units::player::cmp_marker_player::CmpMarkerPlayer;
use crate::units::weapon::cmp_weapon::CmpWeaponHolder;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(
        &mut self,
    ) -> (
        (
            CmpTransform2D,
            CmpTeam,
            Name,
            Mesh3d,
            MeshMaterial3d<StandardMaterial>,
            CmpUnitMovementInput,
            CmpCollisionVolume,
            CmpWeaponHolder,
        ),
        (CmpMarkerPlayer, CmpMarkerCameraTarget, SpatialListener),
    ) {
        let mut mob = self.mob(MobKind::Player);
        mob.1.team = ETeam::Player;

        (
            mob,
            (
                CmpMarkerPlayer,
                CmpMarkerCameraTarget,
                SpatialListener::new(1.5),
            ),
        )
    }
}
