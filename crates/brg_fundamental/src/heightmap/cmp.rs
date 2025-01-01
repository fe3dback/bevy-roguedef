use bevy::prelude::Component;

// TerrainMarkerMesh can be used to mark entities as terrain
// all coord raycasts will filter and use only entities with this flag
#[derive(Component)]
pub struct CmpTerrainMarkerMesh;

// CmpExternalHeightmapDataImporter used to override world heightmap data from
// static entity. Useful for loading pre-calculated level heightmaps
#[derive(Component)]
pub struct CmpExternalHeightmapDataImporter {
    pub width:  u32,
    pub height: u32,
    pub points: Vec<f32>,
}
