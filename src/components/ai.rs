use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect, Default)]
pub struct CmpEnemyMarkerAttackWhenNear {}

#[derive(Component, Reflect)]
pub struct CmpEnemyMarkerMoveToCastleAI {}
