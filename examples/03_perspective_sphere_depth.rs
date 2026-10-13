// An perspective camera looking at two spheres at different depths, shaded by depth in front of a
// gradient. The second render with ray-object intersections, still without light transport.
mod common;

use std::f64::consts::PI;

use common::{Film, Sphere, closest_hit, output_path};
use patina::{
    camera::{Camera, CameraSample, PerspectiveCamera, RenderingSpace},
    math::{Bounds2f, PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, Transform},
};

// Hand rolled coloring for this example. Gives us a gradient.
fn background_color(ray: &PatinaRay3f) -> [u8; 3] {
    let r = ((ray.o.x + 1.0) / 2.0 * 255.0) as u8;
    let g = ((1.0 - ray.o.y) / 2.0 * 255.0) as u8;
    let b = 255;
    [r, g, b]
}

// The depth range of the scene, used to shade every sphere on the same scale: from the front of the
// nearest sphere to the back of the farthest one.
fn depth_range(spheres: &[Sphere]) -> (PatinaFloat, PatinaFloat) {
    let near = spheres
        .iter()
        .map(|s| s.origin.z - s.radius)
        .fold(PatinaFloat::INFINITY, PatinaFloat::min);
    let far = spheres
        .iter()
        .map(|s| s.origin.z + s.radius)
        .fold(PatinaFloat::NEG_INFINITY, PatinaFloat::max);
    (near, far)
}

// Grayscale by depth: white at `near`, black at `far`. Depth is the hit point's z in camera space,
// which is render space here since we render in RenderingSpace::Camera.
fn depth_color(hit: &PatinaPoint3f, (near, far): (PatinaFloat, PatinaFloat)) -> [u8; 3] {
    let brightness = (1.0 - (hit.z - near) / (far - near)).clamp(0.0, 1.0);
    let v = (brightness * 255.0) as u8;
    [v, v, v]
}

fn ray_color(ray: &PatinaRay3f, spheres: &[Sphere], depth: (PatinaFloat, PatinaFloat)) -> [u8; 3] {
    match closest_hit(spheres, ray) {
        Some((_, hit)) => depth_color(&hit.p, depth),
        None => background_color(ray),
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let mut film = Film::new(WIDTH, HEIGHT);

    // A perspective camera at the world origin, rendering in camera space, so render space is
    // camera space.
    let screen_window = Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0));
    let camera = PerspectiveCamera::new(
        &Transform::identity(),
        RenderingSpace::Camera,
        0.0,
        1.0,
        (WIDTH as u32, HEIGHT as u32),
        &screen_window,
        PI / 4.0,
    );

    // The camera looks down +z from the origin and sees x, y in [-1, 1]. Two spheres of the same
    // size: one at z = 3, and one twice as far away, offset so the near sphere partly covers it.
    let spheres = [
        Sphere {
            origin: PatinaPoint3f::new(-0.5, 0.0, 3.0),
            radius: 0.5,
        },
        Sphere {
            origin: PatinaPoint3f::new(0.3, 0.2, 6.0),
            radius: 0.5,
        },
    ];
    let depth = depth_range(&spheres);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sample = CameraSample::new(
                PatinaPoint2f::new(x as f64, y as f64),
                PatinaPoint2f::zero(),
                0.5,
                1.0,
            );
            let color = match camera.generate_ray(&sample) {
                Some(ray) => ray_color(&ray, &spheres, depth),
                None => [0, 0, 0],
            };

            film.set(x, y, color);
        }
    }

    film.save(&output_path("03_perspective_sphere_depth"));
}
