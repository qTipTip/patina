pub mod projective;
pub mod r#trait;

use crate::math::{PatinaFloat, points::PatinaPoint2f, transforms::Transform};

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
pub enum RenderingCoordinateSpace {
    Camera,
    CameraWorld,
    World,
}

impl CameraTransform {
    pub fn new(space: RenderingCoordinateSpace, world_from_camera: &Transform) {
        let world_from_render = match space {
            RenderingCoordinateSpace::Camera => {

            },
            RenderingCoordinateSpace::CameraWorld => todo!(),
            RenderingCoordinateSpace::World => todo!(),
        }
    }
}
