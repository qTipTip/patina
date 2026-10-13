// Helpers shared by the examples. Each example includes this with `mod common;`. Not every example
// uses every helper, hence the allow.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use patina::math::{PatinaFloat, PatinaPoint3f, PatinaRay3f};

pub struct Film {
    width: usize,
    height: usize,
    pixels: Vec<[u8; 3]>,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![[0, 0, 0]; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: [u8; 3]) {
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

    pub fn save(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).expect("failed to create output directory");
        }
        self.to_image().save(path).expect("failed to save image");
        println!("saved {}", path.display());
    }
}

// Where to write the render: the first command line argument if given, otherwise
// `renders/<name>.png`. `just render <name>` passes the path explicitly.
pub fn output_path(name: &str) -> PathBuf {
    std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("renders/{name}.png")))
}

pub struct Sphere {
    pub origin: PatinaPoint3f,
    pub radius: PatinaFloat,
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
