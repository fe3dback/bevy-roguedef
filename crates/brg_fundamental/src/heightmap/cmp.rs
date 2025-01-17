use bevy::prelude::Component;

// TerrainMarkerMesh can be used to mark entities as terrain
// all coord raycasts will filter and use only entities with this flag
#[derive(Component)]
pub struct CmpTerrainMarkerMesh;
