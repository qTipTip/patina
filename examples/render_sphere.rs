use std::path::PathBuf;

use image::{Rgb, RgbImage};
use patina::{
    camera::{
        CameraBase, CameraSample, CameraTransform, RenderingSpace, Shutter,
        projective::orthographic::OrthographicCamera, traits::Camera,
    },
    math::{
        bounding_boxes::Bounds2f, points::PatinaPoint2f, rays::ray3::PatinaRay3f,
        transforms::Transform,
    },
};

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

// Hand rolled coloring for this example. Gives us a gradient.
fn ray_color(ray: &PatinaRay3f) -> [u8; 3] {
    let r = ((ray.o.x + 1.0) / 2.0 * 255.0) as u8;
    let g = ((1.0 - ray.o.y) / 2.0 * 255.0) as u8;
    let b = 255;
    [r, g, b]
}

// Meant to serve as a minimal working render. We should render points in a sphere as white, and
// everything else black.
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let mut film = Film {
        width: WIDTH,
        height: HEIGHT,
        pixels: vec![[0, 0, 0]; WIDTH * HEIGHT],
    };

    // An orthographic camera at the world origin, rendering in camera space, so render space is
    // camera space.
    let base = CameraBase::new(
        CameraTransform::new(RenderingSpace::Camera, &Transform::identity()),
        Shutter::new(0.0, 1.0),
        WIDTH as u32,
        HEIGHT as u32,
    );
    let screen_window = Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0));
    let camera = OrthographicCamera::new(base, &screen_window);

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

    let image = film.to_image();
    image
        .save(PathBuf::from("examples/images/render_sphere.png"))
        .expect("failed to save image");
}
