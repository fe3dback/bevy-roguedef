use crate::components::player::CmpMarkerPlayer;
use crate::prefabs::sup::SupPrefabs;
use bevy::prelude::Transform;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(super) fn player() -> (CmpMarkerPlayer, Transform) {
        (
            CmpMarkerPlayer {},
            Transform::default(),
            // Sprite::from_image()
        )
    }
}
