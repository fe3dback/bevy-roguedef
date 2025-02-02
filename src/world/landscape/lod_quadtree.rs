use brg_core::prelude::{Chunk, V2};

#[derive(Default, Debug)]
pub struct LodQuadTree {
    pos:   V2,
    size:  V2,
    len:   u32,
    child: Option<[Box<LodQuadTree>; 4]>,
}

impl LodQuadTree {
    pub fn new(pos: V2, size: V2, poi: V2) -> Self {
        let quart = size / 4.0;

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

        let mut ind: usize = 0;
        let mut len: u32 = 4;
        let max_dist = size.x;

        for center in child_centers {
            childs[ind].pos = center - quart;
            childs[ind].size = quart * 2.0;

            if center.distance(poi) < max_dist
                && size.x > Chunk::size_m().x * 2.0
                && size.y > Chunk::size_m().y * 2.0
            {
                childs[ind] = Box::new(Self::new(childs[ind].pos, childs[ind].size, poi));
                len += childs[ind].len;
            }

            ind += 1;
        }

        Self {
            pos,
            size,
            len,
            child: Some(childs),
        }
    }

    pub fn leafs(&self) -> Vec<(V2, V2)> {
        let mut result: Vec<(V2, V2)> = Vec::with_capacity(self.len as usize);
        self.leafs_req(&mut result);

        result
    }

    fn leafs_req(&self, collect: &mut Vec<(V2, V2)>) {
        if let Some(childs) = &self.child {
            for child in childs {
                child.leafs_req(collect);
            }

            return;
        }

        collect.push((self.pos, self.size));
    }
}
