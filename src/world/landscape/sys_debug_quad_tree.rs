use bevy::color::palettes::tailwind::YELLOW_700;
use bevy::prelude::Res;
use brg_core::prelude::V2;
use brg_fundamental::prelude::{GizmosX, ResCoords};

use super::lod_quadtree::LodQuadTree;
use super::sup::SupLandscape;

pub fn sys_debug_quad_tree(mut ls: SupLandscape, mut gz: GizmosX, coord: Res<ResCoords>) {
    let height = 0.25;

    let size = ls.heightmap.world_size();
    // todo: update quad tree only on chunk exit
    let tree = LodQuadTree::new(V2::ZERO - (size / 2.0), size, coord.mouse_world_pos);

    for leaf in tree.leafs() {
        let tl = leaf.0;
        let tr = tl + V2::new(leaf.1.x, 0.0);
        let bl = tl + V2::new(0.0, leaf.1.y);
        let br = tl + leaf.1;

        gz.line_custom_height(tl.with_height(height), tr.with_height(height), YELLOW_700);
        gz.line_custom_height(tr.with_height(height), br.with_height(height), YELLOW_700);
        gz.line_custom_height(br.with_height(height), bl.with_height(height), YELLOW_700);
        gz.line_custom_height(bl.with_height(height), tl.with_height(height), YELLOW_700);
    }
}
