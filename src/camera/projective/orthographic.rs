use crate::{
    camera::{CameraBase, projective::ProjectiveTransforms, r#trait::Camera},
    math::{
        bounding_boxes::Bounds2f,
        points::PatinaPoint3f,
        rays::ray3::PatinaRay3f,
        transforms::{Apply, Transform},
        vectors::PatinaVec3f,
    },
};

pub struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,

    camera_dx: PatinaVec3f,
    camera_dy: PatinaVec3f,
}

impl OrthographicCamera {
    pub fn new(base: CameraBase, screen_window: &Bounds2f) -> Self {
        let screen_from_camera = Transform::orthographic(0.0, 1.0);
        let projective_transforms =
            ProjectiveTransforms::new(screen_from_camera, screen_window, base.resolution());

        // The camera_dx and camera_dy vectors tells us how much the camera changes when changing
        // raster position 1 unit x / 1 unit y respectively. Will be used for ray differentials
        // later.
        let camera_dx = projective_transforms
            .camera_from_raster
            .apply(PatinaVec3f::new(1.0, 0.0, 0.0));
        let camera_dy = projective_transforms
            .camera_from_raster
            .apply(PatinaVec3f::new(0.0, 1.0, 0.0));

        Self {
            base,
            projective: projective_transforms,
            camera_dx,
            camera_dy,
        }
    }
}

impl Camera for OrthographicCamera {
    fn generate_ray(
        &self,
        sample: &crate::camera::CameraSample,
    ) -> Option<crate::math::rays::ray3::PatinaRay3f> {
        let p_film = PatinaPoint3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.projective.camera_from_raster.apply(p_film);

        let ray = PatinaRay3f::new(p_camera, PatinaVec3f::new(0.0, 0.0, 1.0));

        // TODO: Modify for depth of field
        Some(ray)
    }

    fn base(&self) -> &CameraBase {
        &self.base
    }
}
