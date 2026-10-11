use crate::{
    math::{
        PatinaFloat, bounding_boxes::Bounds2f, points::PatinaPoint2f, rays::ray3::PatinaRay3f,
        transforms::Transform, vectors::PatinaVec3f,
    },
    sampling::linear::lerp,
};

struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filter_weight: PatinaFloat,
}

// A shutter has an open and close time.
struct Shutter {
    open: PatinaFloat,
    close: PatinaFloat,
}

// Contains shared state between all camera types
struct CameraBase {
    camera_transform: Transform,
    shutter: Shutter,
    resolution_x: u32,
    resolution_y: u32,
}

// Contains transformations needed for for some of the projective cameras (perspective /
// orthograpic).
struct ProjectiveTransforms {
    screen_from_camera: Transform,
    screen_from_raster: Transform,
    camera_from_raster: Transform,
    raster_from_screen: Transform,
}

struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
struct PerspectiveCamera {}

// Defines the common functionality each camera has to implement.
trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f>;
    fn base(&self) -> &CameraBase;
    fn sample_time(&self, u: PatinaFloat) -> PatinaFloat {
        lerp(u, self.base().shutter.open, self.base().shutter.close)
    }
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
