use bevy::color::palettes::tailwind::ROSE_700;
use bevy::prelude::{Camera, Query, Res, ResMut};
use brg_core::prelude::{V2, V3};
use brg_scene::prelude::{EditorFeature, SupEditorFeatures};

use crate::prelude::{
    CmpTransform2D,
    Point,
    ResCoords,
    SupGizmos,
    SupHeightmap,
    SupRayCastMesh,
    TransformHeightKind,
};

pub fn sys_update_active_camera(
    q: Query<(&Camera, &CmpTransform2D)>,
    mut data: ResMut<ResCoords>,
    mut hm: SupHeightmap,
    mut cast: SupRayCastMesh,
    edit_features: SupEditorFeatures,
) {
    if edit_features.has_feature(EditorFeature::GizmosCameraRay) {
        // no update, when gizmo is active
        return;
    }

    for (camera, trm) in &q {
        if !camera.is_active {
            continue;
        }

        data.camera_origin = V3::new(
            trm.position.x,
            trm.position.y,
            match trm.height_kind {
                TransformHeightKind::Absolute => trm.height,
                TransformHeightKind::AboveTerrain => hm.height_at_pos(trm.position) + trm.height,
            },
        );

        let viewport_center = V2::new(
            data.screen_ui_width as f32 / 2.0,
            data.screen_ui_height as f32 / 2.0,
        );

        data.camera_target = cast.ray_cast_from_screen(viewport_center);
        break;
    }
}

pub fn sys_debug_draw_camera_ray(mut gz: SupGizmos, data: Res<ResCoords>) {
    gz.arrow(
        Point::Abs(data.camera_origin),
        Point::Abs(data.camera_target),
        ROSE_700,
    )
}
