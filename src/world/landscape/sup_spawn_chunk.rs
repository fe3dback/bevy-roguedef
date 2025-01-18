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
use super::dto::LoadedChunk;
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
        self.ensure_terrain_material_exist();
        let mesh_lod0_detailed = self.create_mesh(*chunk, EChunkLodLevel::LOD0);
        let mesh_lod1_corners = self.create_mesh(*chunk, EChunkLodLevel::LOD1);
        let Some(material) = self.state.terrain_material.clone() else {
            warn!(
                "cannot spawn landscape chunk {}, terrain material not exist",
                chunk
            );

            return;
        };

        // spawn
        let ent_lod0 = self
            .cmd
            .spawn((
                Name::from(format!("LOD_0 [{}x{}]", chunk.x, chunk.y)),
                CmpTransform2D {
                    position: chunk.position_tl(),
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                Visibility::Visible,
                Mesh3d(mesh_lod0_detailed),
                MeshMaterial3d(material.clone()),
                CmpLandscapeChild {
                    chunk: *chunk,
                    lod:   EChunkLodLevel::LOD0,
                },
            ))
            .id();

        let ent_lod1 = self
            .cmd
            .spawn((
                Name::from(format!("LOD_1 [{}x{}]", chunk.x, chunk.y)),
                CmpTransform2D {
                    position: chunk.position_tl(),
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                Visibility::Hidden,
                Mesh3d(mesh_lod1_corners),
                MeshMaterial3d(material.clone()),
                CmpLandscapeChild {
                    chunk: *chunk,
                    lod:   EChunkLodLevel::LOD1,
                },
            ))
            .id();

        // mark as loaded
        self.cmd.entity(terrain_id).add_child(ent_lod0);
        self.cmd.entity(terrain_id).add_child(ent_lod1);

        self.state.loaded_chunks.insert(
            *chunk,
            LoadedChunk {
                lod0: ent_lod0,
                lod1: ent_lod1,
            },
        );
    }

    pub(super) fn despawn_chunk(&mut self, chunk: &Chunk) {
        let Some(loaded_chunk) = self.state.loaded_chunks.get(chunk) else {
            return;
        };

        self.cmd.entity(loaded_chunk.lod0).despawn_recursive();
        self.cmd.entity(loaded_chunk.lod1).despawn_recursive();
        self.state.loaded_chunks.remove(chunk);
    }
}
