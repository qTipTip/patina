use std::path::PathBuf;

use image::{Rgb, RgbImage};
use patina::math::{PatinaFloat, points::PatinaPoint2f, rays::ray3::PatinaRay3f};

struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filter_weight: PatinaFloat,
}

impl Default for CameraSample {
    fn default() -> Self {
        Self {
            p_film: PatinaPoint2f::zero(),
            p_lens: PatinaPoint2f::zero(),
            time: Default::default(),
            filter_weight: Default::default(),
        }
    }
}

struct Film {
    width: usize,
    height: usize,
    pixels: Vec<[u8; 3]>,
}

impl Film {
    fn set(&mut self, x: usize, y: usize, value: [u8; 3]) {
        debug_assert!(x < self.width && y < self.height);

        let idx = y * self.width + x;
        self.pixels[idx] = value;
    }

    fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = y * self.width + x;
                image.put_pixel(x as u32, y as u32, Rgb(self.pixels[idx]));
            }
        }
        image
    }
}

trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f>;
    fn get_film(&self) -> &Film;
    // Maps u in [0, 1) to [shutter_open, shutter_close).
    fn sample_time(&self, u: PatinaFloat) -> PatinaFloat;
}

// Meant to serve as a minimal working render. We should render points in a sphere as white, and
// everything else black.
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let mut film = Film {
        width: WIDTH,
        height: HEIGHT,
        pixels: Vec::from(&[[0, 0, 0]; WIDTH * HEIGHT]),
    };

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let r = (x as f64 / WIDTH as f64 * 255.0) as u8;
            let g = (y as f64 / HEIGHT as f64 * 255.0) as u8;
            let b = 255;

            film.set(x, y, [r, g, b]);
        }
    }

    let image = film.to_image();
    image
        .save(PathBuf::from("examples/images/render_sphere.jpg"))
        .expect("failed to save image");
}
