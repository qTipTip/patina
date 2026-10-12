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
