use std::path::PathBuf;

use image::{Rgb, RgbImage};
use patina::{
    camera::{Camera, CameraSample, OrthographicCamera, RenderingSpace},
    math::{Bounds2f, PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, Transform},
};

struct Sphere {
    origin: PatinaPoint3f,
    radius: PatinaFloat,
}

impl Sphere {
    // Returns the closest point where the ray hits the sphere, if it hits at all (t >= 0). We'll
    // implement this more generally later.
    pub fn ray_intersection(&self, ray: &PatinaRay3f) -> Option<PatinaPoint3f> {
        // Vector from the sphere's center to the ray's origin (Point - Point = Vector).
        let oc = ray.o - self.origin;

        let a = ray.d.dot(&ray.d);
        let b = 2.0 * oc.dot(&ray.d);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // The two solutions, nearest first. Use the nearest one in front of the ray's origin.
        let sqrt_discriminant = discriminant.sqrt();
        let t0 = (-b - sqrt_discriminant) / (2.0 * a);
        let t1 = (-b + sqrt_discriminant) / (2.0 * a);
        let t = if t0 >= 0.0 {
            t0
        } else if t1 >= 0.0 {
            t1
        } else {
            return None;
        };

        Some(ray.eval(t))
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

// Meant to serve as a minimal working render: a sphere shaded by depth in front of a gradient.
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

    let image = film.to_image();
    image
        .save(PathBuf::from("examples/images/render_sphere.png"))
        .expect("failed to save image");
}
