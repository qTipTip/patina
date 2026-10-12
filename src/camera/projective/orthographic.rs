use crate::camera::{CameraBase, projective::ProjectiveTransforms};

pub struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
