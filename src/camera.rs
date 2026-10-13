mod projective;
mod traits;

// The public camera API: the camera types and the trait they implement. The building blocks they
// share (CameraBase, CameraTransform, Shutter, ProjectiveTransforms) are crate-private.
pub use projective::orthographic::OrthographicCamera;
pub use projective::perspective::PerspectiveCamera;
pub use traits::Camera;

use crate::math::{
    PatinaFloat,
    points::{PatinaPoint2f, PatinaPoint3f},
    transforms::{Apply, Transform},
    vectors::PatinaVec3f,
};

pub struct CameraSample {
    p_film: PatinaPoint2f,  // Point on the film in (x, y)
    _p_lens: PatinaPoint2f, // Point on the lens in (x, y) // TODO: Not used yet
    time: PatinaFloat,
    _filter_weight: PatinaFloat, // TODO: Not used yet
}

impl CameraSample {
    pub fn new(
        p_film: PatinaPoint2f,
        p_lens: PatinaPoint2f,
        time: PatinaFloat,
        filter_weight: PatinaFloat,
    ) -> Self {
        Self {
            p_film,
            _p_lens: p_lens,
            time,
            _filter_weight: filter_weight,
        }
    }
}

// A shutter has an open and close time.
pub(crate) struct Shutter {
    open: PatinaFloat,
    close: PatinaFloat,
}

impl Shutter {
    pub(crate) fn new(open: PatinaFloat, close: PatinaFloat) -> Self {
        debug_assert!(open <= close);
        Self { open, close }
    }
}

// Contains shared state between all camera types. Public only because `Camera::base` returns it;
// it has no public fields or constructor.
pub struct CameraBase {
    camera_transform: CameraTransform, // describes how the camera is placed in the world.
    shutter: Shutter,
    resolution_x: u32,
    resolution_y: u32,
}

impl CameraBase {
    pub(crate) fn new(
        camera_transform: CameraTransform,
        shutter: Shutter,
        resolution_x: u32,
        resolution_y: u32,
    ) -> Self {
        Self {
            camera_transform,
            shutter,
            resolution_x,
            resolution_y,
        }
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.resolution_x, self.resolution_y)
    }
}

// A camera transform helps us switch which coordinate system we perform the rendering in.
pub(crate) struct CameraTransform {
    render_from_camera: Transform, // TODO: Make this an animated transform.
    _world_from_render: Transform,
}

// Camera-space: we obtain lots of accuracy cloest to the camera, but it deteriorates
// for objects further away. Furthermore, acceleration structures are often axis aligned, which
// means that if we rotate the camera (so it's not axis aligned), we're reducing the effectiveness
// of such structures.
//
// CameraWorld-space: Camera is at the origin, and the scene is transalted accordingly. No rotation
// is applied to scene geometry, so bounding boxes stay effective.w
//
// World-space: Loses precision if the camera is placed far away from the origin.
pub enum RenderingSpace {
    Camera,
    CameraWorld,
    World,
}

impl CameraTransform {
    pub(crate) fn new(space: RenderingSpace, world_from_camera: &Transform) -> Self {
        // The world_from_render transform is computed based on which space we are rendering in.
        // TODO: Handle AnimatedTransforms here.
        //
        // Given a point in render space, where is it in the world space?
        let _world_from_render = match space {
            RenderingSpace::Camera => *world_from_camera,
            RenderingSpace::CameraWorld => {
                let p_camera = world_from_camera.apply(PatinaPoint3f::zero());
                Transform::translate(&PatinaVec3f::new(p_camera.x, p_camera.y, p_camera.z))
            }
            RenderingSpace::World => Transform::identity(),
        };

        let render_from_world = _world_from_render.inverse();
        // Given a point in the camera space, where is it in render space?
        let render_from_camera = render_from_world * (*world_from_camera);
        Self {
            render_from_camera,
            _world_from_render,
        }
    }
}

#[cfg(test)]
mod test_camera_transform {
    use approx::assert_abs_diff_eq;

    use crate::{
        camera::{CameraTransform, RenderingSpace},
        math::{
            points::PatinaPoint3f,
            transforms::{Apply, Transform},
            vectors::PatinaVec3f,
        },
    };

    #[test]
    fn test_camera_transforms_camera() {
        let world_from_camera = Transform::rotate_x(0.3)
            * Transform::rotate_z(1.2)
            * Transform::translate(&PatinaVec3f::new(10.0, -1.0, 2.3));
        let c = CameraTransform::new(RenderingSpace::Camera, &world_from_camera);

        // world_from_render * render_from_camera should equal world_from_camera
        assert_abs_diff_eq!(
            world_from_camera,
            c._world_from_render * c.render_from_camera,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_camera_transforms_camera_world() {
        let world_from_camera = Transform::rotate_x(0.3)
            * Transform::rotate_z(1.2)
            * Transform::translate(&PatinaVec3f::new(10.0, -1.0, 2.3));
        let c = CameraTransform::new(RenderingSpace::CameraWorld, &world_from_camera);

        // world_from_render * render_from_camera should equal world_from_camera
        assert_abs_diff_eq!(
            world_from_camera,
            c._world_from_render * c.render_from_camera,
            epsilon = 1e-12
        );

        // For camera world, the origin in render space should map to the camera position in world space , and directions should be
        // unchanged.
        let p_cam = world_from_camera.apply(PatinaPoint3f::zero());
        // The render origin is where the camera is located
        assert_eq!(p_cam, c._world_from_render.apply(PatinaPoint3f::zero()));
        // The camera sits at the render origin
        assert_eq!(
            c.render_from_camera.apply(PatinaPoint3f::zero()),
            PatinaPoint3f::zero()
        );
        // World points are shifted, but not rotated
        let render_from_world = c._world_from_render.inverse();
        let p = PatinaPoint3f::new(3.2, 10.3, -2.3);
        let v = p - p_cam;
        assert_eq!(
            render_from_world.apply(p),
            PatinaPoint3f::new(v.x, v.y, v.z)
        );
        // world directions are unchanged
        let w = PatinaVec3f::new(3.2, 1.2, -2.3);
        assert_eq!(render_from_world.apply(w), w);

        // camera keeps it's world orientation
        let v = PatinaVec3f::new(3.2, 1.2, -2.3);
        assert_eq!(c.render_from_camera.apply(v), world_from_camera.apply(v));
    }

    #[test]
    fn test_camera_transforms_world() {
        let world_from_camera = Transform::rotate_x(0.3)
            * Transform::rotate_z(1.2)
            * Transform::translate(&PatinaVec3f::new(10.0, -1.0, 2.3));
        let c = CameraTransform::new(RenderingSpace::World, &world_from_camera);

        // world_from_render * render_from_camera should equal world_from_camera
        assert_abs_diff_eq!(
            world_from_camera,
            c._world_from_render * c.render_from_camera,
            epsilon = 1e-12
        );

        // For the world render space, the world_from_render should be identity, as render space is
        // world space.
        assert_abs_diff_eq!(c._world_from_render, Transform::identity())
    }
}
