// an orthographic camera generating one ray per pixel, colored by where the ray starts
// on the screen.
mod common;

use common::{Film, output_path};
use patina::{
    camera::{Camera, CameraSample, OrthographicCamera, RenderingSpace},
    math::{Bounds2f, PatinaPoint2f, PatinaRay3f, RenderSpace, Transform},
};

// The screen window spans [-1, 1] x [-1, 1], with screen y pointing up and raster y pointing down,
// so this gives red increasing to the right and green increasing downwards.
fn ray_color(ray: &PatinaRay3f<RenderSpace>) -> [u8; 3] {
    let r = ((ray.o.x + 1.0) / 2.0 * 255.0) as u8;
    let g = ((1.0 - ray.o.y) / 2.0 * 255.0) as u8;
    let b = 255;
    [r, g, b]
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

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sample = CameraSample::new(
                PatinaPoint2f::new(x as f64, y as f64),
                PatinaPoint2f::zero(),
                0.5,
                1.0,
            );
            let color = match camera.generate_ray(&sample) {
                Some(ray) => ray_color(&ray),
                None => [0, 0, 0],
            };

            film.set(x, y, color);
        }
    }

    film.save(&output_path("01_gradient"));
}
