use crate::{
    camera::{CameraBase, CameraSample},
    math::{PatinaFloat, RenderSpace, rays::ray3::PatinaRay3f},
    sampling::linear::lerp,
};

// Defines the common functionality each camera has to implement.
pub trait Camera {
    /// Generates the ray through the sample's point on the film, in render space.
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f<RenderSpace>>;
    fn base(&self) -> &CameraBase;
    // Maps a number in [0, 1) to [open, close).
    fn sample_time(&self, u: PatinaFloat) -> PatinaFloat {
        lerp(u, self.base().shutter.open, self.base().shutter.close)
    }
}
