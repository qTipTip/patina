use std::path::PathBuf;

use image::{Rgb, RgbImage};
use num_traits::ToPrimitive;
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
    pixels: Vec<PatinaFloat>,
}

impl Film {
    fn set(&mut self, x: usize, y: usize, value: PatinaFloat) {
        debug_assert!(x < self.width && y < self.height);

        let idx = y * self.width + x;
        self.pixels[idx] = value;
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
    let (width, height) = (800, 800);

    let mut image = RgbImage::new(width, height);
    for x in 0..width {
        let r = (x as f64 / width as f64 * 255.0) as u8;
        for y in 0..height {
            let g = (y as f64 / height as f64 * 255.0) as u8;
            let b: u8 = 255;

            image.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    image
        .save(PathBuf::from("examples/images/render_sphere.jpg"))
        .expect("failed to save image");
}
