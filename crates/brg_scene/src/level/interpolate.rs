use std::collections::HashMap;

use brg_core::prelude::{Chunk, T_LIB_CONT_ROW_LEN, T_LIB_CONT_SIZE_SQ, V2};

pub fn weighted_fill(key_points: HashMap<V2, f32>, offset: V2) -> [f32; T_LIB_CONT_SIZE_SQ] {
    let mut result = [0.0; T_LIB_CONT_SIZE_SQ];

    let mut tile_x: i32 = 0;
    let mut tile_y: i32 = 0;

    for ind in 0..T_LIB_CONT_SIZE_SQ {
        let tile = offset + V2::new(tile_x as f32, tile_y as f32);

        // calculate weights
        let mut total_value: f32 = 0.0;
        let mut total_weight = 0.0;

        for (key_tile, key_value) in &key_points {
            let distance = key_tile.distance(tile);
            match distance < f32::EPSILON {
                true => {
                    total_value = *key_value;
                    total_weight = 1.0;
                    break;
                }
                false => {
                    let weight = 1.0 / distance;
                    total_value += key_value * weight;
                    total_weight += weight;
                }
            };
        }

        result[ind] = match total_weight < f32::EPSILON {
            true => 0.0,
            false => total_value / total_weight,
        };

        // move cursor
        tile_x += 1;

        if tile_x >= T_LIB_CONT_ROW_LEN as i32 {
            tile_x = 0;
            tile_y += 1;
        }
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn weighted_fill_test() {
        let mut key_points: HashMap<V2, f32> = HashMap::new();
        key_points.insert(V2::new(0.0, 0.0), 0.25);
        key_points.insert(V2::new(14.0, 0.0), 1.0);
        key_points.insert(V2::new(0.0, 14.0), 0.0);
        key_points.insert(V2::new(14.0, 14.0), 1.0);
        key_points.insert(V2::new(7.0, 7.0), 0.0);

        let result = weighted_fill(key_points, V2::ZERO);

        let mut tile_x: i32 = 0;

        for val in &result {
            print!("{}", match *val {
                f32::MIN..0.25 => "░░",
                0.25..0.5 => "▒▒",
                0.5..0.75 => "▓▓",
                0.75..f32::MAX => "██",
                _ => "?",
            });

            tile_x += 1;
            if tile_x >= Chunk::size() as i32 {
                tile_x = 0;
                println!();
            }
        }

        println!("----");

        for val in &result {
            print!(" {:.2} |", *val);

            tile_x += 1;
            if tile_x >= Chunk::size() as i32 {
                tile_x = 0;
                println!();
            }
        }
    }
}
