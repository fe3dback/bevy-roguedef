use bevy::prelude::{ReflectResource, Resource};
use bevy::reflect::Reflect;
use brg_core::prelude::V2;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ResCoords {
    pub world_center:     V2,
    pub screen_world_box: CoordsArea,
    pub screen_ui_pos:    CoordsArea,
    pub screen_ui_width:  u32,
    pub screen_ui_height: u32,
    pub mouse_world_pos:  V2,
    pub mouse_ui_pos:     V2,
}

#[derive(Default, Reflect)]
pub struct CoordsArea {
    pub top_left:     V2,
    pub top_right:    V2,
    pub bottom_left:  V2,
    pub bottom_right: V2,
}
