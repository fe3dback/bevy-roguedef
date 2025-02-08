use bevy::utils::hashbrown::HashSet;
use brg_core::prelude::V2;

use super::dto::MeshIdent;
use super::lod_quadtree::LodQuadLeaf;
use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn ensure_chunks_is_loaded_around_actors(&mut self, point_of_interest: V2) {
        // update indexes
        let prev_quad = self.state.lod_quad_tree.leafs();
        self.update_load_quadtree(point_of_interest);
        let next_quad = self.state.lod_quad_tree.leafs();

        // find changes
        let (created, deleted) = self.diff_between_quads(prev_quad, next_quad);

        // check no need to do anything
        if created.is_empty() && deleted.is_empty() {
            return;
        }

        // delete blocks
        {
            for block in deleted {
                self.despawn_chunk(MeshIdent {
                    pos:   block.position,
                    size:  block.size,
                    depth: block.depth,
                });
            }
        }

        // spawn blocks
        {
            for block in created {
                self.spawn_chunk(
                    MeshIdent {
                        pos:   block.position,
                        size:  block.size,
                        depth: block.depth,
                    },
                    block.transitions(),
                );
            }
        }
    }

    // should return list of
    // - blocks that created since prev
    // - blocks that deleted since prev
    pub(super) fn diff_between_quads(
        &self,
        prev: Vec<LodQuadLeaf>,
        next: Vec<LodQuadLeaf>,
    ) -> (Vec<LodQuadLeaf>, Vec<LodQuadLeaf>) {
        let mut prev_set: HashSet<LodQuadLeaf> = HashSet::with_capacity(prev.len());
        let mut next_set: HashSet<LodQuadLeaf> = HashSet::with_capacity(next.len());

        {
            for e in prev {
                prev_set.insert(e);
            }

            for e in next {
                next_set.insert(e);
            }
        }

        let mut created: Vec<LodQuadLeaf> = Vec::with_capacity(32);
        let mut deleted: Vec<LodQuadLeaf> = Vec::with_capacity(32);

        {
            for e in &next_set {
                if !prev_set.contains(e) {
                    created.push(*e);
                }
            }

            for e in &prev_set {
                if !next_set.contains(e) {
                    deleted.push(*e);
                }
            }
        }

        (created, deleted)
    }
}
