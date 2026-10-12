pub mod projective;
pub mod r#trait;

use crate::math::{
    PatinaFloat,
    points::{PatinaPoint2f, PatinaPoint3f},
    transforms::{Apply, Transform},
    vectors::PatinaVec3f,
};

pub struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filter_weight: PatinaFloat,
}

// A shutter has an open and close time.
pub struct Shutter {
    open: PatinaFloat,
    close: PatinaFloat,
}

// Contains shared state between all camera types
pub struct CameraBase {
    camera_transform: Transform, // describes how the camera is placed in the world.
    shutter: Shutter,
    resolution_x: u32,
    resolution_y: u32,
}

// A camera transform helps us switch which coordinate system we perform the rendering in.
pub struct CameraTransform {
    render_from_camera: Transform, // TODO: Make this an animated transform.
    world_from_render: Transform,
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
    pub fn new(space: RenderingSpace, world_from_camera: &Transform) -> Self {
        // The world_from_render transform is computed based on which space we are rendering in.
        // TODO: Handle AnimatedTransforms here.
        //
        // Given a point in render space, where is it in the world space?
        let world_from_render = match space {
            RenderingSpace::Camera => *world_from_camera,
            RenderingSpace::CameraWorld => {
                let p_camera = world_from_camera.apply(PatinaPoint3f::zero());
                Transform::translate(&PatinaVec3f::new(p_camera.x, p_camera.y, p_camera.z))
            }
            RenderingSpace::World => Transform::identity(),
        };

        let render_from_world = world_from_render.inverse();
        // Given a point in the camera space, where is it in render space?
        let render_from_camera = render_from_world * (*world_from_camera);
        Self {
            render_from_camera,
            world_from_render,
        }
    }
}

#[cfg(test)]
mod test_camera_transform {
    #[test]
    fn test_() {
        todo!();
    }
}
