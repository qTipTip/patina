use crate::{
    camera::{
        self, CameraBase, CameraTransform, RenderingSpace, Shutter,
        projective::ProjectiveTransforms,
    },
    math::{Apply, Bounds2f, PatinaFloat, PatinaPoint3f, PatinaVec3f, Transform},
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
        focal_dist: PatinaFloat,
        lens_radius: PatinaFloat,
    ) -> Self {
        let base = CameraBase::new(
            CameraTransform::new(rendering_space, world_from_camera),
            Shutter::new(shutter_open, shutter_close),
            resolution.0,
            resolution.1,
        );

        Self::from_base(base, screen_window, fov, focal_dist, lens_radius)
    }

    pub(crate) fn from_base(
        base: CameraBase,
        screen_window: &Bounds2f,
        fov: PatinaFloat,
        _focal_dist: PatinaFloat,
        _lens_radius: PatinaFloat,
    ) -> Self {
        let screen_from_camera = Transform::perspective(fov, 0.0, 1.0);

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
