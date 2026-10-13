// an orthographic camera looking at a sphere, shaded by depth in front of a gradient.
// The first render with a ray-object intersection, still without light transport.
mod common;

use common::{Film, output_path};
use patina::{
    camera::{Camera, CameraSample, OrthographicCamera, RenderingSpace},
    math::{Bounds2f, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, Transform},
};

use crate::common::Sphere;

// Hand rolled coloring for this example. Gives us a gradient.
fn background_color(ray: &PatinaRay3f) -> [u8; 3] {
    let r = ((ray.o.x + 1.0) / 2.0 * 255.0) as u8;
    let g = ((1.0 - ray.o.y) / 2.0 * 255.0) as u8;
    let b = 255;
    [r, g, b]
}

// Grayscale by depth: white at the front of the sphere (z = center.z - radius), fading to black at
// its silhouette (z = center.z). Depth is the hit point's z in camera space, which is render space
// here since we render in RenderingSpace::Camera.
fn depth_color(hit: &PatinaPoint3f, sphere: &Sphere) -> [u8; 3] {
    let nearest = sphere.origin.z - sphere.radius;
    let brightness = (1.0 - (hit.z - nearest) / sphere.radius).clamp(0.0, 1.0);
    let v = (brightness * 255.0) as u8;
    [v, v, v]
}

fn ray_color(ray: &PatinaRay3f, sphere: &Sphere) -> [u8; 3] {
    match sphere.ray_intersection(ray) {
        Some(hit) => depth_color(&hit, sphere),
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
    let camera = OrthographicCamera::new(
        &Transform::identity(),
        RenderingSpace::Camera,
        0.0,
        1.0,
        (WIDTH as u32, HEIGHT as u32),
        &screen_window,
    );

    // The camera looks down +z from the origin and sees x, y in [-1, 1]. Place the sphere straight
    // ahead, covering half the image. With no world-to-render transform for geometry yet, the
    // sphere is given directly in render space (= camera space here).
    let sphere = Sphere {
        origin: PatinaPoint3f::new(0.0, 0.0, 3.0),
        radius: 0.5,
    };

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sample = CameraSample::new(
                PatinaPoint2f::new(x as f64, y as f64),
                PatinaPoint2f::zero(),
                0.5,
                1.0,
            );
            let color = match camera.generate_ray(&sample) {
                Some(ray) => ray_color(&ray, &sphere),
                None => [0, 0, 0],
            };

            film.set(x, y, color);
        }
    }

    film.save(&output_path("02_orthographic_sphere_depth"));
}
