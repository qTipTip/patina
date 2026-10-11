use crate::{
    math::{PatinaFloat, rays::ray3::PatinaRay3f, transforms::Transform},
    sampling::linear::lerp,
};

struct CameraSample {}

// A shutter has an open and close time.
struct Shutter {
    pub shutter_open: PatinaFloat,
    pub shutter_close: PatinaFloat,
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
    camera_from_raster: Transform,
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
        lerp(
            u,
            self.base().shutter.shutter_open,
            self.base().shutter.shutter_close,
        )
    }
}
