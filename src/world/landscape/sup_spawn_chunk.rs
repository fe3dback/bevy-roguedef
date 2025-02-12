use bevy::core::Name;
use bevy::log::warn;
use bevy::math::bounding::Aabb3d;
use bevy::prelude::{
    default,
    BuildChildren,
    DespawnRecursiveExt,
    Mesh3d,
    MeshMaterial3d,
    Visibility,
};
use bevy::render::primitives::Aabb;
use brg_core::prelude::{Chunk, V2, V3};
use brg_fundamental::prelude::{CmpMarkerTerrainRayCastSolid, CmpTransform2D, TransformHeightKind};

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

    pub(super) fn update_load_quadtree(&mut self, point_of_interest: V2, dist_to_poe: f32) {
        let size = self.heightmap.world_size();

        self.state.lod_quad_tree = LodQuadTree::new(
            V2::ZERO - (size / 2.0),
            size,
            point_of_interest,
            dist_to_poe,
        );
    }

    pub(super) fn spawn_chunk(&mut self, ident: MeshIdent) {
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
        let mesh = match self.state.created.get(&ident) {
            Some(c) => c,
            None => {
                let c = self.create_mesh(ident);
                self.state.created.insert(ident, c);
                self.state.created.get(&ident).unwrap() // safe, because we insert it one line above
            }
        };
        let Some(material) = self.state.terrain_material.clone() else {
            warn!(
                "cannot spawn landscape block {:?}, terrain material not exist",
                ident
            );

            return;
        };

        let bounding_box_tl = V3::new(0.0, 0.0, mesh.min_abs_height);
        let bounding_box_br = V3::new(
            bounding_box_tl.x + ident.size.x,
            bounding_box_tl.y + ident.size.y,
            mesh.max_abs_height,
        );

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
                Mesh3d(mesh.handle.clone()),
                MeshMaterial3d(material.clone()),
                CmpLandscapeChild { ident },
                CmpMarkerTerrainRayCastSolid,
                Aabb::from_min_max(bounding_box_tl.as_3d(), bounding_box_br.as_3d()),
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
