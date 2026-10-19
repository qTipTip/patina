// A perspective camera looking at two spheres and an ellipsoid, each pixel colored by the surface
// normal at the hit. The first render using the SurfaceInteraction returned by a shape.
//
// The ellipsoid is a sphere scaled differently along each axis. Its normals are only perpendicular
// to its surface if they are transformed as normals (with the inverse transpose), not as vectors,
// so its shading is a visual check of the normal transform.
mod common;

use std::f64::consts::PI;

use common::{Film, closest_hit, output_path, sphere_at};
use patina::{
    camera::{Camera, CameraSample, PerspectiveCamera, RenderingSpace},
    math::{
        Bounds2f, ObjectSpace, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f,
        PatinaRay3f, PatinaVec3f, Transform,
    },
    shapes::Sphere,
};

// Background for rays that hit nothing. A dark gray rather than the gradient, so the normal colors
// stand out.
const BACKGROUND: [u8; 3] = [40, 40, 40];

// Maps each component of a unit normal from [-1, 1] to [0, 255]: x to red, y to green, z to blue.
// The camera looks down +z, so surfaces facing it have a negative z and little blue, and normals
// pointing right are red and pointing up are green.
fn normal_color(n: &PatinaNorm3f) -> [u8; 3] {
    let channel = |c: PatinaFloat| ((c + 1.0) / 2.0 * 255.0) as u8;
    [channel(n.x), channel(n.y), channel(n.z)]
}

fn ray_color(ray: &PatinaRay3f, spheres: &[Sphere]) -> [u8; 3] {
    match closest_hit(spheres, ray) {
        Some(isect) => normal_color(&isect.interaction.n),
        None => BACKGROUND,
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let mut film = Film::new(WIDTH, HEIGHT);

    // A perspective camera at the world origin, rendering in camera space, so render space is
    // camera space and the normals are camera-space normals.
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

    // The two spheres from milestone 03, and an ellipsoid below and to the right: a unit sphere
    // stretched along x and flattened along y.
    let ellipsoid = Sphere::new(
        1.0,
        Transform::translate(&PatinaVec3f::new(0.7, -0.8, 4.0))
            * Transform::<ObjectSpace, ObjectSpace>::scale(0.6, 0.25, 0.4),
        -1.0,
        1.0,
        2.0 * PI,
        false,
    );
    let spheres = [
        sphere_at(PatinaPoint3f::new(-0.5, 0.0, 3.0), 0.5),
        sphere_at(PatinaPoint3f::new(0.3, 0.2, 6.0), 0.5),
        ellipsoid,
    ];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sample_p_film = PatinaPoint2f::new(x as f64, y as f64);
            let sample = CameraSample::new(sample_p_film, PatinaPoint2f::zero(), 0.5, 1.0);
            let color = match camera.generate_ray(&sample) {
                Some(ray) => ray_color(&ray, &spheres),
                None => BACKGROUND,
            };

            film.set(x, y, color);
        }
    }

    film.save(&output_path("04_perspective_sphere_normals"));
}
