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
use brg_core::prelude::V2;
use brg_fundamental::prelude::{CmpTransform2D, TransformHeightKind};

use super::cmp::{CmpLandscapeChild, CmpLandscapeRoot};
use super::dto::MeshIdent;
use super::lod_quadtree::LodQuadTree;
use super::sup::SupLandscape;
use super::sup_mesh::NeighbourSizeTransition;

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

    pub(super) fn update_load_quadtree(&mut self, point_of_interest: V2) {
        let size = self.heightmap.world_size();

        self.state.lod_quad_tree =
            LodQuadTree::new(V2::ZERO - (size / 2.0), size, point_of_interest);
    }

    pub(super) fn spawn_chunk(&mut self, ident: MeshIdent, transition: NeighbourSizeTransition) {
        let terrain_id = self.state.terrain;
        if terrain_id.is_none() {
            warn!(
                "cannot spawn landscape block {:?}, terrain root not exist",
                ident
            );
            return;
        }

        let terrain_id = terrain_id.unwrap();

        // create mesh
        self.ensure_terrain_material_exist();
        let mesh = self.create_mesh(ident, transition);
        let Some(material) = self.state.terrain_material.clone() else {
            warn!(
                "cannot spawn landscape block {:?}, terrain material not exist",
                ident
            );

            return;
        };

        // spawn
        let ent = self
            .cmd
            .spawn((
                Name::from(format!(
                    "[{}x{}] s_{} {:?}",
                    ident.pos.x, ident.pos.y, ident.size, ident.depth,
                )),
                CmpTransform2D {
                    position: ident.pos,
                    height: 0.0,
                    height_kind: TransformHeightKind::Absolute,
                    ..default()
                },
                Visibility::Visible,
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                CmpLandscapeChild { ident },
            ))
            .id();

        // mark as loaded
        self.cmd.entity(terrain_id).add_child(ent);
        self.state.loaded.insert(ident, ent);
    }

    pub(super) fn despawn_chunk(&mut self, ident: MeshIdent) {
        let Some(loaded_ent) = self.state.loaded.get(&ident) else {
            return;
        };

        self.cmd.entity(*loaded_ent).despawn_recursive();
        self.state.loaded.remove(&ident);
    }
}
