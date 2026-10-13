// Helpers shared by the examples. Each example includes this with `mod common;`. Not every example
// uses every helper, hence the allow.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use patina::math::{PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f};

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

// Background for rays that hit nothing. It's based on the pixel position rather than the ray, so it looks
// the same whichever camera generated the ray.
pub fn background_color(p_film: PatinaPoint2f, width: usize, height: usize) -> [u8; 3] {
    let r = (p_film.x / width as PatinaFloat * 255.0) as u8;
    let g = (p_film.y / height as PatinaFloat * 255.0) as u8;
    let b = 255;
    [r, g, b]
}

pub struct Sphere {
    pub origin: PatinaPoint3f,
    pub radius: PatinaFloat,
}

// A ray-sphere hit. the distance along the ray, and the point it hits.
pub struct Hit {
    pub t: PatinaFloat,
    pub p: PatinaPoint3f,
}

impl Sphere {
    // Returns the closest hit in front of the ray's origin with t < t_max, if any.
    pub fn intersect(&self, ray: &PatinaRay3f, t_max: PatinaFloat) -> Option<Hit> {
        // Vector from the sphere's center to the ray's origin (Point - Point = Vector).
        let oc = ray.o - self.origin;

        let a = ray.d.dot(&ray.d);
        let b = 2.0 * oc.dot(&ray.d);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // The two solutions, nearest first. Use the nearest one in [0, t_max).
        let sqrt_discriminant = discriminant.sqrt();
        let t0 = (-b - sqrt_discriminant) / (2.0 * a);
        let t1 = (-b + sqrt_discriminant) / (2.0 * a);
        let t = [t0, t1].into_iter().find(|t| (0.0..t_max).contains(t))?;

        Some(Hit { t, p: ray.eval(t) })
    }

    // Returns the closest point where the ray hits the sphere, if it hits at all (t >= 0).
    pub fn ray_intersection(&self, ray: &PatinaRay3f) -> Option<PatinaPoint3f> {
        self.intersect(ray, PatinaFloat::INFINITY).map(|hit| hit.p)
    }
}

// The closest hit among several spheres, together with the sphere that was hit. Each hit lowers
// t_max, so spheres behind the closest one so far are rejected.
pub fn closest_hit<'a>(spheres: &'a [Sphere], ray: &PatinaRay3f) -> Option<(&'a Sphere, Hit)> {
    let mut closest = None;
    let mut t_max = PatinaFloat::INFINITY;
    for sphere in spheres {
        if let Some(hit) = sphere.intersect(ray, t_max) {
            t_max = hit.t;
            closest = Some((sphere, hit));
        }
    }
    closest
}

// The depth range of the scene, used to shade every sphere on the same scale: from the front of the
// nearest sphere to the back of the farthest one.
pub fn depth_range(spheres: &[Sphere]) -> (PatinaFloat, PatinaFloat) {
    let near = spheres
        .iter()
        .map(|s| s.origin.z - s.radius)
        .fold(PatinaFloat::INFINITY, PatinaFloat::min);
    let far = spheres
        .iter()
        .map(|s| s.origin.z + s.radius)
        .fold(PatinaFloat::NEG_INFINITY, PatinaFloat::max);
    (near, far)
}

// Grayscale by depth: white at `near`, black at `far`. Depth is the hit point's z in render space,
// which is camera depth only when rendering in RenderingSpace::Camera, as the examples do.
pub fn depth_color(hit: &PatinaPoint3f, (near, far): (PatinaFloat, PatinaFloat)) -> [u8; 3] {
    let brightness = (1.0 - (hit.z - near) / (far - near)).clamp(0.0, 1.0);
    let v = (brightness * 255.0) as u8;
    [v, v, v]
}
