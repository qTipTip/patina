use crate::{
    camera::{CameraBase, projective::ProjectiveTransforms, r#trait::Camera},
    math::{bounding_boxes::Bounds2f, transforms::Transform},
};

pub struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}

impl OrthographicCamera {
    pub fn new(base: CameraBase, screen_window: &Bounds2f) -> Self {
        let screen_from_camera = Transform::orthographic(0.0, 1.0);
        let projective_transforms = ProjectiveTransforms::new(
            screen_from_camera,
            screen_window,
            base.resolution_x,
            base.resolution_y,
        );

        Self {
            base,
            projective: projective_transforms,
        }
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
