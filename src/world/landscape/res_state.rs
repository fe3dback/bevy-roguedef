use bevy::prelude::{Entity, Handle, Mesh, Resource};
use bevy::utils::hashbrown::HashMap;

use super::dto::MeshIdent;
use super::lod_quadtree::LodQuadTree;
use super::material::TerrainMaterial;

#[derive(Resource)]
pub(crate) struct ResLandscapeState {
    pub(super) lod_quad_tree:    LodQuadTree,
    pub(super) terrain:          Option<Entity>,
    pub(super) loaded:           HashMap<MeshIdent, Entity>,
    pub(super) meshes:           HashMap<MeshIdent, Handle<Mesh>>,
    pub(super) terrain_material: Option<Handle<TerrainMaterial>>,
}

impl Default for ResLandscapeState {
    fn default() -> Self {
        Self {
            lod_quad_tree:    LodQuadTree::default(),
            terrain:          None,
            loaded:           HashMap::with_capacity(256),
            meshes:           HashMap::with_capacity(256),
            terrain_material: None,
        }
    }
}
