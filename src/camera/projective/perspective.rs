use crate::camera::{CameraBase, projective::ProjectiveTransforms};

pub struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
