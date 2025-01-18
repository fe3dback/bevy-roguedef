use bevy::utils::hashbrown::HashSet;
use brg_core::prelude::{Chunk, Range, VecExt, V2};

use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn ensure_chunks_is_loaded_around_actors(&mut self, actors: Vec<V2>) {
        let half_width = 12;
        let half_height = 12;
        let total_width = half_width * 2 + 1;
        let total_height = half_height * 2 + 1;
        let size = total_width * total_height;

        let mut whitelist: HashSet<Chunk> = HashSet::with_capacity(actors.len() * size);
        let mut should_be_loaded: HashSet<Chunk> = HashSet::with_capacity(actors.len() * size);
        let mut should_be_unloaded: HashSet<Chunk> = HashSet::with_capacity(actors.len() * size);

        // calculate chunks that should be active
        {
            for pos in &actors {
                let chunk = pos.chunk();
                let me_and_neighbors = Range::<Chunk>::new(
                    chunk.x - half_width as i32,
                    chunk.y - half_height as i32,
                    chunk.x + half_width as i32,
                    chunk.y + half_height as i32,
                );

                for neighbor in &me_and_neighbors {
                    whitelist.insert(neighbor);
                }
            }
        }

        // calculate chunks that not in whitelist
        {
            for loaded in self.state.loaded_chunks.keys() {
                if whitelist.contains(loaded) {
                    continue;
                }

                should_be_unloaded.insert(*loaded);
            }

            for to_be_loaded in &whitelist {
                if self.state.loaded_chunks.contains_key(to_be_loaded) {
                    continue;
                }

                should_be_loaded.insert(*to_be_loaded);
            }
        }

        // unload not required chunks
        for c in &should_be_unloaded {
            self.despawn_chunk(c);
        }

        // load required chunks
        for c in &should_be_loaded {
            self.spawn_chunk(c);
        }
    }
}
