use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Mesh3d, Name};
use brg_fundamental::prelude::{CmpMarkerCameraTarget, CmpTransform2D};

use crate::prefabs::sup_prefabs::SupPrefabs;
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
            Name,
            Mesh3d,
            MeshMaterial3d<StandardMaterial>,
            CmpUnitMovementInput,
            CmpWeaponHolder,
        ),
        (CmpMarkerPlayer, CmpMarkerCameraTarget),
    ) {
        let mob = self.mob(MobKind::Player);

        (mob, (CmpMarkerPlayer, CmpMarkerCameraTarget))
    }
}
