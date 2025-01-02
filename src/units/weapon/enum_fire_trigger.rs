use bevy::prelude::Reflect;

#[derive(Reflect, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EWeaponTrigger {
    #[default]
    Released,
    JustPressed,
    Pressed,
    JustReleased,
}
