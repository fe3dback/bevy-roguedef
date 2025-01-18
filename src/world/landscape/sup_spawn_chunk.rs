use bevy::core::Name;
use bevy::log::warn;
use bevy::prelude::{
    default,
    BuildChildren,
    DespawnRecursiveExt,
    Mesh3d,
    MeshMaterial3d,
    Visibility,
};
use brg_core::prelude::{BlockPosition, Chunk};
use brg_fundamental::prelude::{CmpTransform2D, TransformHeightKind};

use super::cmp::{CmpLandscapeChild, CmpLandscapeRoot};
use super::enum_lod_level::EChunkLodLevel;
use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn spawn_terrain(&mut self) {
        let id = self
            .cmd
            .spawn((
                Name::from("landscape root"),
                CmpTransform2D {
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                CmpLandscapeRoot,
                Visibility::Visible,
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

    pub(super) fn spawn_chunk(&mut self, lod: EChunkLodLevel, chunk: Chunk) {
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
        self.ensure_terrain_material_exist();
        let mesh = self.create_mesh(chunk, lod);
        let Some(material) = self.state.terrain_material.clone() else {
            warn!(
                "cannot spawn landscape chunk {}, terrain material not exist",
                chunk
            );

            return;
        };

        // spawn
        let ent = self
            .cmd
            .spawn((
                Name::from(format!(
                    "[{}x{}] lod_{}",
                    chunk.x,
                    chunk.y,
                    match lod {
                        EChunkLodLevel::LOD0 => 0,
                        EChunkLodLevel::LOD1 => 1,
                    }
                )),
                CmpTransform2D {
                    position: chunk.position_tl(),
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                Visibility::Visible,
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                CmpLandscapeChild { chunk, lod },
            ))
            .id();

        // mark as loaded
        self.cmd.entity(terrain_id).add_child(ent);
        self.state
            .loaded_chunks
            .get_mut(&lod)
            .unwrap()
            .insert(chunk, ent);
    }

    pub(super) fn despawn_chunk(&mut self, lod: EChunkLodLevel, chunk: Chunk) {
        let Some(loaded_chunks) = self.state.loaded_chunks.get(&lod) else {
            return;
        };

        let Some(loaded_ent) = loaded_chunks.get(&chunk) else {
            return;
        };

        self.cmd.entity(*loaded_ent).despawn_recursive();
        self.state
            .loaded_chunks
            .get_mut(&lod)
            .unwrap()
            .remove(&chunk);
    }
}
