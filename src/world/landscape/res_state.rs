use bevy::prelude::{Entity, Handle, Mesh, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::{V2, V3};

use super::dto::MeshIdent;
use super::lod_quadtree::LodQuadTree;
use super::material::TerrainMaterial;

#[derive(Resource)]
pub(crate) struct ResLandscapeState {
    pub(super) lod_point_of_interest: V2,
    pub(super) lod_last_origin:       V3,
    pub(super) lod_quad_tree:         LodQuadTree,
    pub(super) terrain:               Option<Entity>,
    pub(super) loaded:                HashMap<MeshIdent, Entity>,
    pub(super) meshes:                HashMap<MeshIdent, Handle<Mesh>>,
    pub(super) terrain_material:      Option<Handle<TerrainMaterial>>,
}

impl Default for ResLandscapeState {
    fn default() -> Self {
        Self {
            lod_point_of_interest: V2::ZERO,
            lod_last_origin:       V3::ZERO,
            lod_quad_tree:         LodQuadTree::default(),
            terrain:               None,
            loaded:                HashMap::with_capacity(256),
            meshes:                HashMap::with_capacity(256),
            terrain_material:      None,
        }
    }
}
