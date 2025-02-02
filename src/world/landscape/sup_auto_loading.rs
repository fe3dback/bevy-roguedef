use bevy::utils::hashbrown::{HashMap, HashSet};
use brg_core::prelude::{BlockPosition, Chunk, Range, VecExt, V2};
use strum::{EnumCount, IntoEnumIterator};

use super::enum_lod_level::EChunkLodLevel;
use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn ensure_chunks_is_loaded_around_actors(&mut self, actors: Vec<V2>) {
        let half_width = 32;
        let half_height = 32;
        let total_width = half_width * 2 + 1;
        let total_height = half_height * 2 + 1;
        let size = total_width * total_height;

        let mut staged_chunks: HashMap<EChunkLodLevel, HashSet<Chunk>> =
            HashMap::with_capacity(EChunkLodLevel::COUNT);
        let mut should_be_loaded: HashMap<EChunkLodLevel, HashSet<Chunk>> =
            HashMap::with_capacity(EChunkLodLevel::COUNT);
        let mut should_be_unloaded: HashMap<EChunkLodLevel, HashSet<Chunk>> =
            HashMap::with_capacity(EChunkLodLevel::COUNT);

        for lod in EChunkLodLevel::iter() {
            staged_chunks.insert(lod, HashSet::with_capacity(size));
            should_be_loaded.insert(lod, HashSet::with_capacity(size / 3));
            should_be_unloaded.insert(lod, HashSet::with_capacity(size / 3));
        }

        let mut selected_chunks: HashSet<Chunk> = HashSet::with_capacity(actors.len() * size);

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
                    selected_chunks.insert(neighbor);
                }
            }
        }

        // split active chunks by lod levels and stage to loading
        {
            for chunk in selected_chunks {
                let mut min_dist = 4096.0;

                for pos in &actors {
                    let dist = pos.distance(chunk.position_center());
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }

                let lod = if min_dist <= (Chunk::size() * 4) as f32 {
                    EChunkLodLevel::LOD0
                } else {
                    EChunkLodLevel::LOD1
                };

                staged_chunks.get_mut(&lod).unwrap().insert(chunk);
            }
        }

        // calculate chunks that not in whitelist
        {
            for (lod, already_loaded) in &self.state.loaded_chunks {
                for chunk in already_loaded.keys() {
                    if staged_chunks.get(lod).unwrap().contains(chunk) {
                        continue;
                    }

                    should_be_unloaded.get_mut(lod).unwrap().insert(*chunk);
                }
            }

            for (lod, staged) in &staged_chunks {
                for chunk in staged {
                    if self.state.is_chunk_loaded(*chunk, *lod) {
                        continue;
                    }

                    should_be_loaded.get_mut(lod).unwrap().insert(*chunk);
                }
            }
        }

        // unload not required chunks
        for (lod, chunks) in &should_be_unloaded {
            for chunk in chunks {
                self.despawn_chunk(*lod, *chunk);
            }
        }

        // load required chunks
        for (lod, chunks) in &should_be_loaded {
            for chunk in chunks {
                self.spawn_chunk(*lod, *chunk);
            }
        }
    }
}
