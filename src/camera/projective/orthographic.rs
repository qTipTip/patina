use crate::camera::{CameraBase, projective::ProjectiveTransforms};

pub struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}

impl OrthographicCamera {
    pub fn new(base: CameraBase, projective: ProjectiveTransforms) -> Self {
        Self { base, projective }
    }
}
