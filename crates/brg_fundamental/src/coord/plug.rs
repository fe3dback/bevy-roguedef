use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, PreUpdate, Update};
use brg_scene::prelude::{has_editor_feature, EditorFeature, GameSystemSet};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // 
            .register_type::<super::res_coords::ResCoords>()
            .insert_resource(super::res_coords::ResCoords::default())
            .add_systems(PreUpdate, (
                super::sys_screen::update_world_coords,
                super::sys_cameras::sys_update_active_camera,
            ).chain())
            .add_systems(Update, super::sys_cameras::sys_debug_draw_camera_ray
                .in_set(GameSystemSet::GizmosDraw)
                .run_if(has_editor_feature(EditorFeature::GizmosCameraRay))
            )
        //-
        ;
    }
}
