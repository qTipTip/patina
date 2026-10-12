use crate::camera::{CameraBase, projective::ProjectiveTransforms, r#trait::Camera};

pub struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}

impl OrthographicCamera {
    pub fn new(base: CameraBase, projective: ProjectiveTransforms) -> Self {
        Self { base, projective }
    }
}

impl Camera for OrthographicCamera {
    fn generate_ray(
        &self,
        sample: &crate::camera::CameraSample,
    ) -> Option<crate::math::rays::ray3::PatinaRay3f> {
        todo!()
    }

    fn base(&self) -> &CameraBase {
        todo!()
    }
}
