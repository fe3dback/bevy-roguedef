use bevy::prelude::Component;

use super::dto::MeshIdent;

#[derive(Component)]
pub struct CmpLandscapeRoot;

#[derive(Component)]
#[allow(unused)]
pub struct CmpLandscapeChild {
    pub ident: MeshIdent,
}
