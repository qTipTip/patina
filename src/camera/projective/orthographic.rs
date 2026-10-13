use crate::{
    camera::{
        CameraBase, CameraSample, CameraTransform, RenderingSpace, Shutter,
        projective::ProjectiveTransforms, traits::Camera,
    },
    math::{
        PatinaFloat,
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

    _camera_dx: PatinaVec3f,
    _camera_dy: PatinaVec3f,
}

impl OrthographicCamera {
    // Creates an orthographic camera placed in the world by `world_from_camera`, which renders in
    // `rendering_space` and generates rays at times in [shutter_open, shutter_close).
    pub fn new(
        world_from_camera: &Transform,
        rendering_space: RenderingSpace,
        shutter_open: PatinaFloat,
        shutter_close: PatinaFloat,
        resolution: (u32, u32),
        screen_window: &Bounds2f,
    ) -> Self {
        let base = CameraBase::new(
            CameraTransform::new(rendering_space, world_from_camera),
            Shutter::new(shutter_open, shutter_close),
            resolution.0,
            resolution.1,
        );
        Self::from_base(base, screen_window)
    }

    pub(crate) fn from_base(base: CameraBase, screen_window: &Bounds2f) -> Self {
        let screen_from_camera = Transform::orthographic(0.0, 1.0);
        let projective_transforms =
            ProjectiveTransforms::new(screen_from_camera, screen_window, base.resolution());

        // The camera_dx and camera_dy vectors tells us how much the camera changes when changing
        // raster position 1 unit x / 1 unit y respectively. Will be used for ray differentials
        // later.
        let _camera_dx = projective_transforms
            .camera_from_raster
            .apply(PatinaVec3f::new(1.0, 0.0, 0.0));
        let _camera_dy = projective_transforms
            .camera_from_raster
            .apply(PatinaVec3f::new(0.0, 1.0, 0.0));

        Self {
            base,
            projective: projective_transforms,
            _camera_dx,
            _camera_dy,
        }
    }
}

impl Camera for OrthographicCamera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f> {
        let p_film = PatinaPoint3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.projective.camera_from_raster.apply(p_film);

        // Construct a ray with the given sample time
        let ray = PatinaRay3f::new(
            p_camera,
            PatinaVec3f::new(0.0, 0.0, 1.0),
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
mod test_orthographic_camera {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        camera::{
            CameraBase, CameraSample, CameraTransform, RenderingSpace, Shutter,
            projective::orthographic::OrthographicCamera, traits::Camera,
        },
        math::{
            bounding_boxes::Bounds2f,
            points::{PatinaPoint2f, PatinaPoint3f},
            transforms::{Apply, Transform},
            vectors::PatinaVec3f,
        },
    };

    // Test camera is configured with rendering space = camera, transform = identity, so camera is
    // at the world origin.
    fn get_simple_test_camera() -> OrthographicCamera {
        // Camera setup:
        let t = Transform::identity();
        let b = CameraBase {
            camera_transform: CameraTransform::new(RenderingSpace::Camera, &t),
            shutter: Shutter {
                open: 0.0,
                close: 1.0,
            },
            resolution_x: 100,
            resolution_y: 50,
        };
        let screen_window =
            Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0));

        OrthographicCamera::from_base(b, &screen_window)
    }

    fn get_moved_test_camera(space: RenderingSpace) -> OrthographicCamera {
        // Camera setup: Rotate 90 degrees around the y-axis, so the camera's z-axis points down the
        // world x-axis, then translate to (-10, 0, 0).
        //
        // Remember that a camera generates a ray in render-space.
        let t = Transform::translate(&PatinaVec3f::new(-10.0, 0.0, 0.0))
            * Transform::rotate_y(PI / 2.0);
        let b = CameraBase {
            camera_transform: CameraTransform::new(space, &t),
            shutter: Shutter {
                open: 2.0,
                close: 3.0,
            },
            resolution_x: 100,
            resolution_y: 50,
        };
        let screen_window =
            Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0));

        OrthographicCamera::from_base(b, &screen_window)
    }

    #[test]
    fn test_camera_ray_dxs() {
        let c = get_simple_test_camera();

        assert_relative_eq!(c._camera_dx, PatinaVec3f::new(2.0 / 100.0, 0.0, 0.0));
        assert_relative_eq!(c._camera_dy, PatinaVec3f::new(0.0, -2.0 / 50.0, 0.0));
    }
    // The orthographic camera is in some sense the simplest camera to reason about, as all render
    // rays are parallel and point out of the camera (0, 0, z).
    #[test]
    fn test_camera_ray_generation_top_left() {
        let camera = get_simple_test_camera();
        let sample = CameraSample {
            p_film: PatinaPoint2f::new(0.0, 0.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };

        // The above sample should generate a ray with origin (-1, 1, 0), and direction (0, 0, 1);
        let r = camera
            .generate_ray(&sample)
            .expect("Should generate a valid ray");

        assert_relative_eq!(r.o, PatinaPoint3f::new(-1.0, 1.0, 0.0));
        assert_relative_eq!(r.d, PatinaVec3f::new(0.0, 0.0, 1.0));
        assert_relative_eq!(r.t, 0.5);
    }

    #[test]
    fn test_camera_ray_generation_middle() {
        let camera = get_simple_test_camera();
        let sample = CameraSample {
            p_film: PatinaPoint2f::new(50.0, 25.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };

        // The above sample should generate a ray with origin (0.0, 0.0, 0), and direction (0, 0, 1);
        let r = camera
            .generate_ray(&sample)
            .expect("Should generate a valid ray");

        assert_relative_eq!(r.o, PatinaPoint3f::new(0.0, 0.0, 0.0));
        assert_relative_eq!(r.d, PatinaVec3f::new(0.0, 0.0, 1.0));
        assert_relative_eq!(r.t, 0.5);
    }

    #[test]
    fn test_camera_ray_generation_non_trivial() {
        let camera = get_moved_test_camera(RenderingSpace::World);

        let sample_middle = CameraSample {
            p_film: PatinaPoint2f::new(50.0, 25.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };

        let r = camera
            .generate_ray(&sample_middle)
            .expect("Should generate a valid ray");

        // The generated ray in world space should have origin (-10, 0.0, 0.0) as the sample is in
        // the middle of the orthographic camera, and z_near = 0.
        //
        // The direction should be (1, 0, 0) as we've rotated the camera 90 degrees around the
        // y-axis, so z-axis now points down the x-axis.
        assert_relative_eq!(r.o, PatinaPoint3f::new(-10.0, 0.0, 0.0));
        assert_relative_eq!(r.d, PatinaVec3f::new(1.0, 0.0, 0.0));
        assert_relative_eq!(r.t, 2.5);

        let sample_top_left = CameraSample {
            p_film: PatinaPoint2f::new(0.0, 0.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };

        let r = camera
            .generate_ray(&sample_top_left)
            .expect("Should generate a valid ray");
        // The generated ray in world space should have origin (-10, 1.0, 1.0) as the sample is in
        // the top left of the orthographic camera, and z_near = 0.
        //
        // The direction should be (1, 0, 0) as we've rotated the camera 90 degrees around the
        // y-axis, so z-axis now points down the x-axis.
        assert_relative_eq!(r.o, PatinaPoint3f::new(-10.0, 1.0, 1.0));
        assert_relative_eq!(r.d, PatinaVec3f::new(1.0, 0.0, 0.0));
        assert_relative_eq!(r.t, 2.5);
    }

    // The same moved camera, rendered in each rendering space. The generated rays are in render
    // space, so they differ per mode, but taking them back to world space must give the same ray.
    #[test]
    fn test_camera_ray_generation_rendering_spaces() {
        let sample_middle = CameraSample {
            p_film: PatinaPoint2f::new(50.0, 25.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };
        let sample_top_left = CameraSample {
            p_film: PatinaPoint2f::new(0.0, 0.0),
            _p_lens: PatinaPoint2f::zero(),
            time: 0.5,
            _filter_weight: 0.0,
        };

        // The rays in world space, which are the same regardless of rendering space.
        let world_middle_o = PatinaPoint3f::new(-10.0, 0.0, 0.0);
        let world_top_left_o = PatinaPoint3f::new(-10.0, 1.0, 1.0);
        let world_d = PatinaVec3f::new(1.0, 0.0, 0.0);

        // (name, space, render-space origin of middle ray, of top-left ray, render-space direction)
        let cases = [
            // Render space is world space.
            (
                "World",
                RenderingSpace::World,
                world_middle_o,
                world_top_left_o,
                world_d,
            ),
            // Render space is world space translated so the camera is at the origin. The camera
            // is still rotated relative to it.
            (
                "CameraWorld",
                RenderingSpace::CameraWorld,
                PatinaPoint3f::new(0.0, 0.0, 0.0),
                PatinaPoint3f::new(0.0, 1.0, 1.0),
                world_d,
            ),
            // Render space is camera space, so the rays are those of an unmoved camera.
            (
                "Camera",
                RenderingSpace::Camera,
                PatinaPoint3f::new(0.0, 0.0, 0.0),
                PatinaPoint3f::new(-1.0, 1.0, 0.0),
                PatinaVec3f::new(0.0, 0.0, 1.0),
            ),
        ];

        for (name, space, render_middle_o, render_top_left_o, render_d) in cases {
            eprintln!("checking rendering space: {name}");
            let camera = get_moved_test_camera(space);
            let world_from_render = camera.base.camera_transform._world_from_render;

            for (sample, render_o, world_o) in [
                (&sample_middle, render_middle_o, world_middle_o),
                (&sample_top_left, render_top_left_o, world_top_left_o),
            ] {
                let r = camera
                    .generate_ray(sample)
                    .expect("Should generate a valid ray");

                assert_relative_eq!(r.o, render_o);
                assert_relative_eq!(r.d, render_d);
                assert_relative_eq!(r.t, 2.5);

                let r_world = world_from_render.apply(r);
                assert_relative_eq!(r_world.o, world_o);
                assert_relative_eq!(r_world.d, world_d);
                assert_relative_eq!(r_world.t, 2.5);
            }
        }
    }
}
