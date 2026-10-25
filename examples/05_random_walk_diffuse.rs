// The first render with light transport: the scene from milestone 04 with diffuse materials, on a
// large ground disk under a uniform sky, rendered with the random walk integrator.
mod common;

use std::f64::consts::PI;

use common::{output_path, sphere_at};
use patina::{
    camera::{PerspectiveCamera, RenderingSpace, film::Film},
    integration::{integrators::render, randomwalk::RandomWalkIntegrator},
    materials::Material,
    math::{
        Bounds2f, ObjectSpace, PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaVec3f, RenderSpace,
        Transform,
    },
    primitive::{Aggregate, Primitive},
    shapes::{Disk, Shape, Sphere},
    spectrum::SampledSpectrum,
};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SAMPLES_PER_PIXEL: u32 = 64;
const MAX_DEPTH: u32 = 5;

fn diffuse(shape: impl Shape + 'static, rgb: [PatinaFloat; 3]) -> Primitive {
    Primitive::new(
        Box::new(shape),
        Material::Diffuse {
            reflectance: SampledSpectrum::new(rgb),
        },
    )
}

fn main() {
    // The camera from milestones 02-04: at the origin, rendering in camera space.
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

    // The objects from milestone 04: two spheres and an ellipsoid (a unit sphere stretched along x
    // and flattened along y).
    let ellipsoid = Sphere::new(
        1.0,
        Transform::translate(&PatinaVec3f::new(0.7, -0.8, 4.0))
            * Transform::<ObjectSpace, ObjectSpace>::scale(0.6, 0.25, 0.4),
        -1.0,
        1.0,
        2.0 * PI,
        false,
    );

    // The ground: a large disk at the bottom of the ellipsoid (y = -1.05), so the ellipsoid rests on
    // it. A disk lies in its object space's xy-plane; rotating by -90 degrees around x turns its
    // normal from +z to +y, pointing up.
    let render_from_ground: Transform<RenderSpace, ObjectSpace> =
        Transform::translate(&PatinaVec3f::new(0.0, -1.05, 0.0))
            * Transform::<ObjectSpace, ObjectSpace>::rotate_x(-PI / 2.0);
    let ground = Disk::new(0.0, 100.0, 0.0, 2.0 * PI, render_from_ground, false);

    let aggregate = Aggregate::new(vec![
        diffuse(ground, [0.5, 0.5, 0.5]),
        diffuse(
            sphere_at(PatinaPoint3f::new(-0.5, 0.0, 3.0), 0.5),
            [0.8, 0.25, 0.2],
        ),
        diffuse(
            sphere_at(PatinaPoint3f::new(0.3, 0.2, 6.0), 0.5),
            [0.8, 0.8, 0.8],
        ),
        diffuse(ellipsoid, [0.2, 0.35, 0.8]),
    ]);
    let sky = SampledSpectrum::new([0.75, 0.85, 1.0]);
    let integrator = RandomWalkIntegrator::new(aggregate, sky, MAX_DEPTH);

    let mut film = Film::new(WIDTH, HEIGHT);
    render(&integrator, &camera, &mut film, SAMPLES_PER_PIXEL, 0);

    let path = output_path("05_random_walk_diffuse");
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).expect("failed to create output directory");
    }
    film.save(&path).expect("failed to save image");
    println!("saved {}", path.display());
}
