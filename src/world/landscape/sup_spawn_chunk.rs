use bevy::core::Name;
use bevy::log::warn;
use bevy::prelude::{default, BuildChildren, DespawnRecursiveExt, MaterialMeshBundle, Mesh3d};
use brg_core::prelude::{BlockPosition, Chunk};
use brg_fundamental::prelude::{CmpTransform2D, TransformHeightKind};

use super::cmp::{CmpLandscapeChild, CmpLandscapeRoot};
use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn spawn_terrain(&mut self) {
        let id = self
            .cmd
            .spawn((
                Name::from("landscape root"),
                CmpTransform2D::default(),
                CmpLandscapeRoot,
            ))
            .id();

        self.state.terrain = Some(id);
    }

    pub(super) fn despawn_terrain(&mut self) {
        if let Some(id) = self.state.terrain {
            self.cmd.entity(id).despawn_recursive();
            self.state.terrain = None;
        }
    }

    pub(super) fn spawn_chunk(&mut self, chunk: &Chunk) {
        let terrain_id = self.state.terrain;
        if terrain_id.is_none() {
            warn!(
                "cannot spawn landscape chunk {}, terrain root not exist",
                chunk
            );
            return;
        }

        let terrain_id = terrain_id.unwrap();

        // create mesh
        // let mesh = self.create_mesh(*chunk); // todo

        // spawn
        let ent_id = self
            .cmd
            .spawn((
                Name::from(format!("chunk [{}x{}]", chunk.x, chunk.y)),
                CmpTransform2D {
                    position: chunk.position_tl(),
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                // MaterialMeshBundle {
                //
                // },
                // Mesh3d
                CmpLandscapeChild { chunk: *chunk },
            ))
            .id();

        // mark as loaded
        self.cmd.entity(terrain_id).add_child(ent_id);
        self.state.loaded_chunks.insert(*chunk, ent_id);
    }

    pub(super) fn despawn_chunk(&mut self, chunk: &Chunk) {
        let Some(ent_id) = self.state.loaded_chunks.get(chunk) else {
            return;
        };

        self.cmd.entity(*ent_id).despawn_recursive();
        self.state.loaded_chunks.remove(chunk);
    }
}
