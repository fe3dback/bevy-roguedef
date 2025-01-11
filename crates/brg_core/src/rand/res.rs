use std::marker::PhantomData;

use bevy::prelude::{Resource, TypePath};
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::prelude::types::Angle;

#[derive(Resource)]
pub struct ResRandomSource<T: TypePath> {
    rnd:     ChaCha8Rng,
    _marker: PhantomData<T>,
}

impl<T: TypePath> ResRandomSource<T> {
    pub fn new() -> ResRandomSource<T> {
        ResRandomSource::<T> {
            rnd:     ChaCha8Rng::from_entropy(),
            _marker: PhantomData::default(),
        }
    }

    #[inline]
    pub fn rand_roll_dices(&mut self, dices: u32, dice_faces: u32) -> u32 {
        let mut result: u32 = 0;

        for _ in 0..dices {
            let dice_result = self.rnd.next_u32() % dice_faces + 1;
            result += dice_result;
        }

        result
    }

    #[inline]
    pub fn rand_int32_in_range(&mut self, from: i32, to: i32) -> i32 {
        let (from, to) = if from > to { (to, from) } else { (from, to) };
        let local = i32::abs(self.rnd.next_u32() as i32 % (to - from));

        local + from
    }

    #[inline]
    pub fn rand_element<'v, E>(&mut self, vec: &'v Vec<E>) -> Option<&'v E> {
        let size = vec.len();
        let index = self.rand_int32_in_range(0, size as i32);

        Some(&vec[index as usize])
    }

    #[inline]
    pub fn rand_angle(&mut self) -> Angle {
        f32::to_radians(self.rand_int32_in_range(0, 360) as f32)
    }
}
