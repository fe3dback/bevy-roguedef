use std::hash::{Hash, Hasher};

use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::{Chunk, V2};

use super::sup_mesh::{NeighbourSizeTransition, Side};

#[derive(Default, Debug)]
pub struct LodQuadTree {
    pos:   V2,
    size:  V2,
    len:   u32,
    quad:  Quad,
    depth: u8,
    child: Option<[Box<LodQuadTree>; 4]>,
}

#[derive(Copy, Clone, Debug)]
pub struct LodQuadLeaf {
    pub depth:      u8,
    pub position:   V2,
    pub size:       V2,
    /// in order: top, bottom, left, right
    pub neighbours: [u8; 4],
}

#[derive(Debug)]
struct NeighbourSearchRange {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl NeighbourSearchRange {
    pub fn contains(&self, p: V2) -> bool {
        (p.x >= self.min_x && p.x < self.max_x) && (p.y >= self.min_y && p.y < self.max_y)
    }
}

impl LodQuadLeaf {
    pub fn transitions(&self) -> NeighbourSizeTransition {
        let is_top = self.neighbours[0] < self.depth;
        let is_bottom = self.neighbours[1] < self.depth;
        let is_left = self.neighbours[2] < self.depth;
        let is_right = self.neighbours[3] < self.depth;

        let sides = is_top as u8 + is_bottom as u8 + is_left as u8 + is_right as u8;

        match sides {
            0 => NeighbourSizeTransition::None,
            1 => NeighbourSizeTransition::OneSide(Self::transition_one_side(
                is_top, is_bottom, is_left, is_right,
            )),
            2 => {
                let (side1, side2) =
                    Self::transition_two_sides(is_top, is_bottom, is_left, is_right);
                NeighbourSizeTransition::TwoSides(side1, side2)
            }
            _ => unreachable!(),
        }
    }

    fn transition_one_side(t: bool, b: bool, l: bool, r: bool) -> Side {
        if t {
            return Side::Top;
        }
        if b {
            return Side::Bottom;
        }
        if l {
            return Side::Left;
        }
        if r {
            return Side::Right;
        }

        unreachable!()
    }

    fn transition_two_sides(t: bool, b: bool, l: bool, r: bool) -> (Side, Side) {
        let side1 = Self::find_side_except(t, b, l, r, None);
        let side2 = Self::find_side_except(t, b, l, r, Some(side1));

        (side1, side2)
    }

    fn find_side_except(t: bool, b: bool, l: bool, r: bool, exclude: Option<Side>) -> Side {
        let ex_t = exclude.map(|x| x == Side::Top).unwrap_or(false);
        let ex_b = exclude.map(|x| x == Side::Bottom).unwrap_or(false);
        let ex_l = exclude.map(|x| x == Side::Left).unwrap_or(false);
        let ex_r = exclude.map(|x| x == Side::Right).unwrap_or(false);

        if t && !ex_t {
            return Side::Top;
        }

        if b && !ex_b {
            return Side::Bottom;
        }

        if l && !ex_l {
            return Side::Left;
        }

        if r && !ex_r {
            return Side::Right;
        }

        unreachable!()
    }
}

impl Hash for LodQuadLeaf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.depth.hash(state);
        self.size.hash(state);
    }
}

impl PartialEq<Self> for LodQuadLeaf {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.depth == other.depth && self.size == other.size
    }
}

impl Eq for LodQuadLeaf {
}

#[derive(Default, Debug, Copy, Clone)]
enum Quad {
    #[default]
    TL,
    TR,
    BL,
    BR,
}

impl LodQuadTree {
    pub fn new(pos: V2, size: V2, poi: V2) -> Self {
        if size == V2::ZERO {
            return LodQuadTree::default();
        }

        let quart = size / 4.0;

        // in order: TL, TR, BL, BR
        let child_centers = [
            pos + V2::new(quart.x, quart.y),
            pos + V2::new(quart.x * 3.0, quart.y),
            pos + V2::new(quart.x, quart.y * 3.0),
            pos + V2::new(quart.x * 3.0, quart.y * 3.0),
        ];

        let mut childs: [Box<LodQuadTree>; 4] = [
            Box::new(LodQuadTree::default()),
            Box::new(LodQuadTree::default()),
            Box::new(LodQuadTree::default()),
            Box::new(LodQuadTree::default()),
        ];

        let mut len: u32 = 4;
        let max_dist = size.x;

        let depth_x = (size.x.log2() - Chunk::size_m().x.log2()) as u8;
        let depth_y = (size.y.log2() - Chunk::size_m().y.log2()) as u8;
        let depth = depth_x.max(depth_y);

        let mut created_self = Self {
            pos,
            size,
            len,
            quad: Quad::TL,
            depth,
            child: None,
        };

        for (ind, center) in child_centers.into_iter().enumerate() {
            let child_quad = match ind {
                0 => Quad::TL,
                1 => Quad::TR,
                2 => Quad::BL,
                3 => Quad::BR,
                _ => unreachable!(),
            };

            childs[ind].quad = child_quad;
            childs[ind].pos = center - quart;
            childs[ind].size = quart * 2.0;
            childs[ind].depth = depth - 1;

            if center.distance(poi) < max_dist
                && size.x > Chunk::size_m().x * 2.0
                && size.y > Chunk::size_m().y * 2.0
            {
                childs[ind] = Box::new(Self::new(childs[ind].pos, childs[ind].size, poi));
                len += childs[ind].len;
            }
        }

        created_self.child = Some(childs);
        created_self
    }

    pub fn leafs(&self) -> Vec<LodQuadLeaf> {
        let mut result: Vec<LodQuadLeaf> = Vec::with_capacity(self.len as usize);
        self.leafs_req(&mut result);
        Self::leafs_populate_neighbours(&mut result);

        result
    }

    fn leafs_req(&self, collect: &mut Vec<LodQuadLeaf>) {
        if let Some(childs) = &self.child {
            for child in childs {
                child.leafs_req(collect);
            }

            return;
        }

        collect.push(LodQuadLeaf {
            depth:      self.depth,
            position:   self.pos,
            size:       self.size,
            neighbours: [0; 4],
        });
    }

    fn leafs_populate_neighbours(leafs: &mut Vec<LodQuadLeaf>) {
        let mut centers: HashMap<V2, u8> = HashMap::with_capacity(leafs.len());
        for leaf in leafs.iter() {
            centers.insert(leaf.position + (leaf.size / 2.0), leaf.depth);
        }

        for leaf in leafs {
            let sides = [Side::Top, Side::Bottom, Side::Left, Side::Right];
            for side in sides {
                let search_range = Self::neighbour_search_range(leaf, side);
                let mut min_distance = f32::INFINITY;
                let mut min_depth = u8::MAX;
                let center = leaf.position + (leaf.size / 2.0);

                for (other_center, depth) in centers.iter() {
                    if search_range.contains(*other_center) {
                        let dist = center.distance(*other_center);
                        if dist < min_distance {
                            min_distance = dist;
                            min_depth = *depth;
                        }
                    }
                }

                leaf.neighbours[side as usize] = min_depth;
            }
        }
    }

    fn neighbour_search_range(leaf: &LodQuadLeaf, side: Side) -> NeighbourSearchRange {
        let d_size = leaf.size * 2.0;

        let tl = leaf.position;
        let tr = tl + V2::new(leaf.size.x, 0.0);
        let bl = tl + V2::new(0.0, leaf.size.y);
        let br = tl + V2::new(leaf.size.x, leaf.size.y);

        // find search range
        match side {
            Side::Top => NeighbourSearchRange {
                min_x: tl.x - 1.0,
                max_x: tr.x + 1.0,
                min_y: tl.y - d_size.y,
                max_y: tl.y,
            },
            Side::Bottom => NeighbourSearchRange {
                min_x: bl.x - 1.0,
                max_x: br.x + 1.0,
                min_y: bl.y,
                max_y: bl.y + d_size.y,
            },
            Side::Left => NeighbourSearchRange {
                min_x: tl.x - d_size.x,
                max_x: tl.x,
                min_y: tl.y - 1.0,
                max_y: bl.y + 1.0,
            },
            Side::Right => NeighbourSearchRange {
                min_x: tr.x,
                max_x: tr.x + d_size.x,
                min_y: tr.y - 1.0,
                max_y: br.y + 1.0,
            },
        }
    }
}
