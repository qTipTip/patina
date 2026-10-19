// An orthographic camera looking at two spheres at different depths, shaded by depth in front of a
// gradient. The first render with ray-object intersections, still without light transport.
//
// Both spheres have the same radius, and with an orthographic projection they appear the same size
// even though one is twice as far away. Compare with the perspective camera, where the far one
// shrinks (foreshortening).
mod common;

use common::{
    Film, background_color, closest_hit, depth_color, depth_range, output_path, sphere_at,
};
use patina::{
    camera::{Camera, CameraSample, OrthographicCamera, RenderingSpace},
    math::{
        Bounds2f, PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, RenderSpace, Transform,
    },
    shapes::Sphere,
};

// `p_film` and `resolution` are only used for the background, which is based on the pixel position.
fn ray_color(
    ray: &PatinaRay3f<RenderSpace>,
    p_film: PatinaPoint2f,
    resolution: (usize, usize),
    spheres: &[Sphere],
    depth: (PatinaFloat, PatinaFloat),
) -> [u8; 3] {
    match closest_hit(spheres, ray) {
        Some(isect) => depth_color(&isect.interaction.p, depth),
        None => background_color(p_film, resolution.0, resolution.1),
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let mut film = Film::new(WIDTH, HEIGHT);

    // An orthographic camera at the world origin, rendering in camera space, so render space is
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

    // The camera looks down +z from the origin and sees x, y in [-1, 1]. Two spheres of the same
    // size: one at z = 3, and one twice as far away, offset so the near sphere partly covers it.
    // With no world-to-render transform for geometry yet, the spheres are given directly in render
    // space (= camera space here).
    let spheres = [
        sphere_at(PatinaPoint3f::new(-0.5, 0.0, 3.0), 0.5),
        sphere_at(PatinaPoint3f::new(0.3, 0.2, 6.0), 0.5),
    ];
    let depth = depth_range(&spheres);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sample_p_film = PatinaPoint2f::new(x as f64, y as f64);
            let sample = CameraSample::new(sample_p_film, PatinaPoint2f::zero(), 0.5, 1.0);
            let color = match camera.generate_ray(&sample) {
                Some(ray) => ray_color(&ray, sample_p_film, (WIDTH, HEIGHT), &spheres, depth),
                None => [0, 0, 0],
            };

            film.set(x, y, color);
        }
    }

    film.save(&output_path("02_orthographic_sphere_depth"));
}
