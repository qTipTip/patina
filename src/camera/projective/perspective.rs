use crate::camera::{CameraBase, projective::ProjectiveTransforms};

#[allow(dead_code)]
pub(crate) struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
