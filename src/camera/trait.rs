use crate::{
    camera::{CameraBase, CameraSample},
    math::{PatinaFloat, rays::ray3::PatinaRay3f},
    sampling::linear::lerp,
};

// Defines the common functionality each camera has to implement.
trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f>;
    fn base(&self) -> &CameraBase;
    fn sample_time(&self, u: PatinaFloat) -> PatinaFloat {
        lerp(u, self.base().shutter.open, self.base().shutter.close)
    }
}
