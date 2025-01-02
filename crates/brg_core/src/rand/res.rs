use bevy::prelude::Resource;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::prelude::types::Angle;

#[derive(Resource)]
pub struct ResRandomSource {
    rnd: ChaCha8Rng,
}

impl Default for ResRandomSource {
    fn default() -> Self {
        Self {
            rnd: ChaCha8Rng::from_entropy(),
        }
    }
}

impl ResRandomSource {
    #[inline]
    pub fn rand_roll_dices(&mut self, dices: u16, dice_faces: u16) -> u32 {
        let mut result: u32 = 0;

        for _ in 0..dices {
            let dice_result = self.rnd.next_u32() % dice_faces as u32 + 1;
            result += dice_result;
        }

        result
    }

    #[inline]
    pub fn rand_int32_in_range(&mut self, from: i32, to: i32) -> i32 {
        let (from, to) = if from > to { (to, from) } else { (from, to) };

        self.rnd.next_u32() as i32 % (to - from) + from
    }

    #[inline]
    pub fn rand_angle(&mut self) -> Angle {
        f32::to_radians(self.rand_int32_in_range(0, 360) as f32)
    }
}
