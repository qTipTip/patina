use crate::{
    camera::{
        self, Camera, CameraBase, CameraTransform, RenderingSpace, Shutter,
        projective::ProjectiveTransforms,
    },
    math::{Apply, Bounds2f, PatinaFloat, PatinaPoint3f, PatinaRay3f, PatinaVec3f, Transform},
};

pub struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,

    _camera_dx: PatinaVec3f,
    _camera_dy: PatinaVec3f,
}

impl PerspectiveCamera {
    pub fn new(
        world_from_camera: &Transform,
        rendering_space: RenderingSpace,
        shutter_open: PatinaFloat,
        shutter_close: PatinaFloat,
        resolution: (u32, u32),
        screen_window: &Bounds2f,
        fov: PatinaFloat,
    ) -> Self {
        let base = CameraBase::new(
            CameraTransform::new(rendering_space, world_from_camera),
            Shutter::new(shutter_open, shutter_close),
            resolution.0,
            resolution.1,
        );

        Self::from_base(base, screen_window, fov)
    }

    pub(crate) fn from_base(base: CameraBase, screen_window: &Bounds2f, fov: PatinaFloat) -> Self {
        let screen_from_camera = Transform::perspective(fov, 1.0e-2, 1000.0);

        let projective_transforms =
            ProjectiveTransforms::new(screen_from_camera, screen_window, base.resolution());

        let camera_dx = projective_transforms
            .camera_from_raster
            .apply(PatinaPoint3f::new(1.0, 0.0, 0.0))
            - projective_transforms
                .camera_from_raster
                .apply(PatinaPoint3f::zero());
        let camera_dy = projective_transforms
            .camera_from_raster
            .apply(PatinaPoint3f::new(0.0, 1.0, 0.0))
            - projective_transforms
                .camera_from_raster
                .apply(PatinaPoint3f::zero());

        Self {
            base,
            projective: projective_transforms,
            _camera_dx: camera_dx,
            _camera_dy: camera_dy,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: &camera::CameraSample) -> Option<crate::math::PatinaRay3f> {
        let p_film = PatinaPoint3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.projective.camera_from_raster.apply(p_film);
        let v_camera = PatinaVec3f::new(p_camera.x, p_camera.y, p_camera.z).normalize();

        // Construct a ray with the given sample time
        let ray = PatinaRay3f::new(
            PatinaPoint3f::zero(),
            v_camera,
            self.sample_time(sample.time),
        );

        // TODO: Modify for depth of field

        // Take the ray, and render it from the camera using the camera transform.
        Some(self.base.camera_transform.render_from_camera.apply(ray))
    }

    fn base(&self) -> &CameraBase {
        &self.base
    }
}
