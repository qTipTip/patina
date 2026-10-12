pub mod projective;
pub mod r#trait;

use crate::{
    math::{
        PatinaFloat, bounding_boxes::Bounds2f, points::PatinaPoint2f, rays::ray3::PatinaRay3f,
        transforms::Transform, vectors::PatinaVec3f,
    },
    sampling::linear::lerp,
};

struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filter_weight: PatinaFloat,
}

// A shutter has an open and close time.
struct Shutter {
    open: PatinaFloat,
    close: PatinaFloat,
}

// Contains shared state between all camera types
struct CameraBase {
    camera_transform: Transform, // describes how the camera is placed in the world.
    shutter: Shutter,
    resolution_x: u32,
    resolution_y: u32,
}

// Contains transformations needed for for some of the projective cameras (perspective /
// orthograpic).
struct ProjectiveTransforms {
    screen_from_camera: Transform,
    screen_from_raster: Transform,
    camera_from_raster: Transform,
    raster_from_screen: Transform,
}

struct OrthographicCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}
struct PerspectiveCamera {
    base: CameraBase,
    projective: ProjectiveTransforms,
}

impl ProjectiveTransforms {
    fn new(
        screen_from_camera: Transform,
        screen_window: &Bounds2f,
        raster_width: u32,
        raster_height: u32,
    ) -> Self {
        // Scales and translates screen into NDC (0, 1)-coordinates.
        let ndc_from_screen = Transform::scale(
            1.0 / (screen_window.p_max.x - screen_window.p_min.x),
            1.0 / (screen_window.p_max.y - screen_window.p_min.y),
            1.0,
        ) * Transform::translate(&PatinaVec3f::new(
            -screen_window.p_min.x,
            -screen_window.p_max.y,
            0.0,
        ));
        // Scales the NDC coordinates into raster coordinates.
        let raster_from_ndc = Transform::scale(
            raster_width as PatinaFloat,
            -(raster_height as PatinaFloat),
            1.0,
        );

        let raster_from_screen = raster_from_ndc * ndc_from_screen;
        let screen_from_raster = raster_from_screen.inverse();
        let camera_from_raster = screen_from_camera.inverse() * screen_from_raster;

        Self {
            screen_from_camera,
            raster_from_screen,
            screen_from_raster,
            camera_from_raster,
        }
    }
}

#[cfg(test)]
mod test_projections {
    use crate::{
        camera::ProjectiveTransforms,
        math::{
            bounding_boxes::Bounds2f,
            points::{PatinaPoint2f, PatinaPoint3f},
            transforms::{Apply, Transform},
        },
    };

    #[test]
    fn test_raster_transforms() {
        let p = ProjectiveTransforms::new(
            Transform::identity(),
            &Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0)),
            100,
            50,
        );

        // In the following, note how upper_left in screen space = (-1, 1, z), as the y-coord is "inverted".
        //
        // Given the point (-1, 1, 0) in screen space, it should map to (0, 0, 0) in raster space.
        let screen_p = PatinaPoint3f::new(-1.0, 1.0, 0.0);
        let raster_p = p.raster_from_screen.apply(screen_p);

        assert_eq!(raster_p, PatinaPoint3f::new(0.0, 0.0, 0.0));
        // and back:
        assert_eq!(screen_p, p.screen_from_raster.apply(raster_p));

        // Given the point (-1, 1, 1) in screen space, it should map to (0, 0, 1) in raster space.
        let screen_p = PatinaPoint3f::new(-1.0, 1.0, 1.0);
        let raster_p = p.raster_from_screen.apply(screen_p);

        assert_eq!(raster_p, PatinaPoint3f::new(0.0, 0.0, 1.0));
        // and back:
        assert_eq!(screen_p, p.screen_from_raster.apply(raster_p));

        // Given the point (1, -1, 0.5) in screen space, it should map to (100, 50, 0.5) in raster space.
        let screen_p = PatinaPoint3f::new(1.0, -1.0, 0.5);
        let raster_p = p.raster_from_screen.apply(screen_p);

        assert_eq!(raster_p, PatinaPoint3f::new(100.0, 50.0, 0.5));
        // and back:
        assert_eq!(screen_p, p.screen_from_raster.apply(raster_p));
    }
}
