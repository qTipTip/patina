use crate::{
    camera::{
        Camera, CameraBase, CameraSample, CameraTransform, RenderingSpace, Shutter,
        projective::ProjectiveTransforms,
    },
    math::{
        Apply, Bounds2f, CameraSpace, PatinaFloat, PatinaPoint3f, PatinaRay3f, PatinaVec3f,
        RasterSpace, RenderSpace, Transform, WorldSpace,
    },
};

pub struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,

    _camera_dx: PatinaVec3f<CameraSpace>,
    _camera_dy: PatinaVec3f<CameraSpace>,
}

impl PerspectiveCamera {
    pub fn new(
        world_from_camera: &Transform<WorldSpace, CameraSpace>,
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
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f<RenderSpace>> {
        // Given a point on the film (x, y), we find the corresponding point in the camera space on
        // the near-plane (z = 0).
        let p_film: PatinaPoint3f<RasterSpace> =
            PatinaPoint3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.projective.camera_from_raster.apply(p_film);

        // The direction is then given by the vector between the origin (0, 0, 0) and p_camera (as
        // all rays originate at (0, 0, 0)).
        let v_camera = PatinaVec3f::from(p_camera).normalize();

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

#[cfg(test)]
mod test_perspective_camera {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        camera::{Camera, CameraSample, PerspectiveCamera},
        math::{Bounds2f, PatinaPoint2f, PatinaPoint3f, PatinaVec3f, Transform},
    };

    fn get_test_camera_simple() -> PerspectiveCamera {
        PerspectiveCamera::new(
            &Transform::identity(),
            crate::camera::RenderingSpace::Camera,
            0.0,
            0.1,
            (100, 50),
            &Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0)),
            PI / 2.0,
        )
    }

    #[test]
    fn test_camera_ray_generation_middle() {
        let camera = get_test_camera_simple();
        let sample = CameraSample::new(
            PatinaPoint2f::new(50.0, 25.0),
            PatinaPoint2f::zero(),
            0.5,
            0.0,
        );

        let r = camera
            .generate_ray(&sample)
            .expect("Should generate a valid ray");

        // The ray should point directly down the camera (z = 1).
        assert_relative_eq!(r.o, PatinaPoint3f::zero());
        assert_relative_eq!(r.d, PatinaVec3f::new(0.0, 0.0, 1.0));
    }
}
