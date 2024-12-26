use super::prelude::Tile;
use super::range::Range;
use super::selection::Selection;
use super::tile::VecExt;
use crate::vectors::prelude::V2;

pub fn tiles_on_world(pos_tl: V2, pos_br: V2) -> Range {
    Range::new(
        pos_tl.x as i32 - 1,
        pos_tl.y as i32 - 1,
        pos_br.x as i32 + 1,
        pos_br.y as i32 + 1,
    )
}

pub fn select_tiles_with_center_inside_radius(center: V2, radius: f32) -> Selection {
    let center_tile = center.tile();

    let radius_tiles_ext = (radius + 1.0).ceil() as i32;
    let possible = Range::new(
        center_tile.x - radius_tiles_ext,
        center_tile.y - radius_tiles_ext,
        center_tile.x + radius_tiles_ext,
        center_tile.y + radius_tiles_ext,
    );

    let mut matched_tiles: Vec<Tile> = Vec::with_capacity(possible.len());
    for tile in &possible {
        let dist = tile.position_center().distance(center);
        if dist <= radius {
            matched_tiles.push(tile);
        }
    }

    return Selection::from_vec(matched_tiles);
}

pub fn select_n_tiles_around_position(center: V2, width: i32, height: i32) -> Range {
    let center_tile = center.tile();

    if width <= 1 && height <= 1 {
        return Range::new(center_tile.x, center_tile.y, center_tile.x, center_tile.y);
    }

    let is_even_width = width % 2 == 0;
    let is_even_height = height % 2 == 0;

    let grab_min_x = match is_even_width {
        false => (width - 1) / 2,
        true => (width / 2) - 1,
    };
    let grab_min_y = match is_even_height {
        false => (height - 1) / 2,
        true => (height / 2) - 1,
    };
    let grab_max_x = match is_even_width {
        false => (width - 1) / 2,
        true => width / 2,
    };
    let grab_max_y = match is_even_height {
        false => (height - 1) / 2,
        true => height / 2,
    };

    return Range::new(
        center_tile.x - grab_min_x,
        center_tile.y - grab_min_y,
        center_tile.x + grab_max_x,
        center_tile.y + grab_max_y,
    );
}
