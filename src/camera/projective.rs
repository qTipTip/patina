use crate::math::{
    PatinaFloat, bounding_boxes::Bounds2f, transforms::Transform, vectors::PatinaVec3f,
};

pub mod orthographic;
pub mod perspective;

// Contains transformations needed for for some of the projective cameras (perspective /
// orthograpic).
pub struct ProjectiveTransforms {
    screen_from_camera: Transform,
    screen_from_raster: Transform,
    camera_from_raster: Transform,
    raster_from_screen: Transform,
}

impl ProjectiveTransforms {
    fn new(
        screen_from_camera: Transform,
        screen_window: &Bounds2f,
        raster_width: u32,
        raster_height: u32,
    ) -> Self {
        // Scales and translates screen into NDC (0, 1)-coordinates.
        let ndc_from_screen = Transform::scale(
            1.0 / (screen_window.p_max.x - screen_window.p_min.x),
            1.0 / (screen_window.p_max.y - screen_window.p_min.y),
            1.0,
        ) * Transform::translate(&PatinaVec3f::new(
            -screen_window.p_min.x,
            -screen_window.p_max.y,
            0.0,
        ));
        // Scales the NDC coordinates into raster coordinates.
        let raster_from_ndc = Transform::scale(
            raster_width as PatinaFloat,
            -(raster_height as PatinaFloat),
            1.0,
        );

        let raster_from_screen = raster_from_ndc * ndc_from_screen;
        let screen_from_raster = raster_from_screen.inverse();
        let camera_from_raster = screen_from_camera.inverse() * screen_from_raster;

        Self {
            screen_from_camera,
            raster_from_screen,
            screen_from_raster,
            camera_from_raster,
        }
    }
}
