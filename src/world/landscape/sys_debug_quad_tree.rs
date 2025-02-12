use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::tailwind::{
    GRAY_900,
    GREEN_500,
    LIME_300,
    ORANGE_500,
    RED_500,
    ROSE_700,
    YELLOW_500,
};
use brg_core::prelude::{V2, V3};
use brg_fundamental::prelude::{Point, SupGizmos};

use super::sup::SupLandscape;
use super::sup_mesh::{NeighbourSizeTransition, Side};
pub fn sys_debug_quad_tree(ls: SupLandscape, mut gz: SupGizmos) {
    let height = 0.5;
    for block in ls.state.lod_quad_tree.leafs() {
        let tl = block.position;
        let tr = tl + V2::new(block.size.x, 0.0);
        let bl = tl + V2::new(0.0, block.size.y);
        let br = tl + block.size;

        let tl = tl.with_height(height);
        let tr = tr.with_height(height);
        let bl = bl.with_height(height);
        let br = br.with_height(height);

        let top = tl.mid_point(tr);
        let right = tr.mid_point(br);
        let bottom = bl.mid_point(br);
        let left = tl.mid_point(bl);

        let color = match block.depth {
            0 => LIME_300,
            1 => GREEN_500,
            2 => YELLOW_500,
            3 => ORANGE_500,
            4 => RED_500,
            5 => ROSE_700,
            6 => GRAY_900,
            _ => BLACK,
        };

        // draw borders
        {
            gz.line(Point::Rel(tl), Point::Rel(tr), color);
            gz.line(Point::Rel(tr), Point::Rel(br), color);
            gz.line(Point::Rel(br), Point::Rel(bl), color);
            gz.line(Point::Rel(bl), Point::Rel(tl), color);
        }

        // draw transitions to lower level
        {
            let arrow_size = 3.0 * (block.depth + 1) as f32;
            let mut draw_side_trx = |side: Side| match side {
                Side::Top => gz.arrow(
                    Point::Rel(top + V3::new(0.0, 5.0, 0.0)),
                    Point::Rel(top - V3::new(0.0, arrow_size, 0.0)),
                    color,
                ),
                Side::Right => gz.arrow(
                    Point::Rel(right - V3::new(5.0, 0.0, 0.0)),
                    Point::Rel(right + V3::new(arrow_size, 0.0, 0.0)),
                    color,
                ),
                Side::Bottom => gz.arrow(
                    Point::Rel(bottom - V3::new(0.0, 5.0, 0.0)),
                    Point::Rel(bottom + V3::new(0.0, arrow_size, 0.0)),
                    color,
                ),
                Side::Left => gz.arrow(
                    Point::Rel(left + V3::new(5.0, 0.0, 0.0)),
                    Point::Rel(left - V3::new(arrow_size, 0.0, 0.0)),
                    color,
                ),
            };

            let transition = block.transitions();
            match transition {
                NeighbourSizeTransition::None => {}
                NeighbourSizeTransition::OneSide(side) => draw_side_trx(side),
                NeighbourSizeTransition::TwoSides(side1, side2) => {
                    draw_side_trx(side1);
                    draw_side_trx(side2);
                }
            }
        }
    }
}
