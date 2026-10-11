use std::path::PathBuf;

use image::{Rgb, RgbImage};
use patina::{
    math::{
        PatinaFloat, points::{PatinaPoint2f, PatinaPoint3f}, rays::ray3::PatinaRay3f, transforms::Transform,
    }, sampling::linear::lerp,
};

struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filter_weight: PatinaFloat,
}

struct ProjectiveCamera {
    film: Film,
    width: u32,
    height: u32,

    shutter_open: PatinaFloat,
    shutter_closed: PatinaFloat,

    raster_from_screen: Transform,
    screen_from_raster: Transform,
    camera_from_raster: Transform,
}

// We need a set various coordinate systems to meaningfully discuss cameras.
//
// Screen space: defined on the film plane. Camera projects objects from camera space onto the film
// plane, and the parts within the "screen window" is visible. Points at the near plane map to z =
// 0, and points at the far plane map to z = 1.
//
// NDC Space (Normalized device coordinates): Coordinates for actual image being rendered, (x, y):
// [0, 1]x[0, 1]. Same z-values as screen space.
//
// Raster space: Same as NDC space, but scaled to the resolution of the image, so (x, y): [0,
// width]x[0, height].
//
// We need a set of transformations to convert between these coordinate spaces.
impl ProjectiveCamera {
    pub fn new() {
            
    }
}

impl Camera for ProjectiveCamera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<PatinaRay3f> {
        let p_film = PatinaPoint3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = camera_from_raster(p_film);
        None
    }

    fn get_film(&self) -> &Film {
        &self.film
    }

    fn sample_time(&self, u: PatinaFloat) -> PatinaFloat {
        debug_assert!(self.shutter_open < self.shutter_closed);
        lerp(u, self.shutter_open, self.shutter_closed)
    }
}

// This method takes a raster point (on the film), and transforms it into camera space.
fn camera_from_raster(p_film: patina::math::tuples::tuple3::Tuple3<f64, patina::math::tuples::TypePoint>) -> _ {
    let t = Transform::scale(1.0 / , y, z)
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
        .save(PathBuf::from("examples/images/render_sphere.png"))
        .expect("failed to save image");
}
