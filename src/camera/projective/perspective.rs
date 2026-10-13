use crate::camera::{CameraBase, projective::ProjectiveTransforms};

pub(crate) struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
