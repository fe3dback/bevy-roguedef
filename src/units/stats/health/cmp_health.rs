use bevy::prelude::{Component, Reflect};
use bevy_health_bar3d::prelude::{BarSettings, Percentage};

#[derive(Component, Reflect)]
#[require(BarSettings<CmpHealth>)]
pub struct CmpHealth {
    pub(super) amount:       f32,
    pub(super) total:        f32,
    pub(super) alive:        bool,
    pub(super) invulnerable: bool,
}

impl Default for CmpHealth {
    fn default() -> Self {
        Self::new_splat(100.0)
    }
}

impl CmpHealth {
    pub fn new_splat(amount: f32) -> Self {
        Self::new(amount, amount, false)
    }

    pub fn new(amount: f32, total: f32, invulnerable: bool) -> Self {
        debug_assert!(total > 0.0);
        let amount = Self::clamp(amount, total);

        Self {
            amount,
            total,
            alive: amount > 0.0,
            invulnerable,
        }
    }

    fn clamp(value: f32, total: f32) -> f32 {
        value.clamp(0.0, total)
    }

    pub fn take_damage(&mut self, amount: f32) {
        if !self.alive {
            return;
        }

        match self.amount > amount {
            true => {
                self.amount -= amount;
            }
            false => {
                self.amount = 0.0;
                self.alive = false;
            }
        }
    }

    #[inline]
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    #[inline]
    pub fn is_invulnerable(&self) -> bool {
        self.invulnerable
    }
}

impl Percentage for CmpHealth {
    fn value(&self) -> f32 {
        self.amount / self.total
    }
}
