use bevy::prelude::{default, EntityCommands, SpatialListener};
use brg_fundamental::prelude::CmpMarkerCameraTarget;

use super::prelude::MobSettings;
use super::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::ETeam;
use crate::units::mobs::enum_mob_type::MobKind;
use crate::units::player::cmp_marker_player::CmpMarkerPlayer;
use crate::world::landscape::cmp_actor_initiator::CmpLandscapeLoadActorInitiator;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(&mut self) -> EntityCommands {
        self.mob(
            &MobSettings {
                kind: MobKind::Player,
                team: ETeam::Player,
                ..default()
            },
            (
                CmpMarkerPlayer,
                CmpMarkerCameraTarget,
                CmpLandscapeLoadActorInitiator,
                SpatialListener::new(1.5),
            ),
        )
    }
}
