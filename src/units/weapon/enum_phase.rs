use bevy::prelude::Reflect;

#[derive(Reflect, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum EShootingPhase {
    StandBy,
    Shooting,
    Reloading,
}
