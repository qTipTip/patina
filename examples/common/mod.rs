// Helpers shared by the examples. Each example includes this with `mod common;`. Not every example
// uses every helper, hence the allow.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use std::f64::consts::PI;

use patina::{
    math::{
        PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, PatinaVec3f, RenderSpace, Transform,
    },
    shapes::{Shape, ShapeIntersection, Sphere},
};

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

// A full sphere of the given radius, centered at `center`. The library's spheres are centered at
// the origin of their object space, and placed with a render_from_object transform.
pub fn sphere_at(center: PatinaPoint3f<RenderSpace>, radius: PatinaFloat) -> Sphere {
    let render_from_object = Transform::translate(&PatinaVec3f::from(center));
    Sphere::new(radius, render_from_object, -radius, radius, 2.0 * PI, false)
}

// The closest intersection among the spheres. Each hit lowers t_max, so spheres behind the closest
// one so far are rejected.
pub fn closest_hit(
    spheres: &[Sphere],
    ray: &PatinaRay3f<RenderSpace>,
) -> Option<ShapeIntersection> {
    let mut closest = None;
    let mut t_max = PatinaFloat::INFINITY;
    for sphere in spheres {
        if let Some(isect) = sphere.intersect(ray, Some(t_max)) {
            t_max = isect.t_hit;
            closest = Some(isect);
        }
    }
    closest
}

// The depth range of the scene, from the spheres' bounding boxes: the front of the nearest sphere to
// the back of the farthest one.
pub fn depth_range(spheres: &[Sphere]) -> (PatinaFloat, PatinaFloat) {
    let bounds = spheres.iter().map(|s| s.bounds());
    let near = bounds
        .clone()
        .map(|b| b.p_min.z)
        .fold(PatinaFloat::INFINITY, PatinaFloat::min);
    let far = bounds
        .map(|b| b.p_max.z)
        .fold(PatinaFloat::NEG_INFINITY, PatinaFloat::max);
    (near, far)
}

// Grayscale by depth: white at `near`, black at `far`. Depth is the hit point's z in render space,
// which is camera depth only when rendering in RenderingSpace::Camera, as the examples do.
pub fn depth_color(
    hit: &PatinaPoint3f<RenderSpace>,
    (near, far): (PatinaFloat, PatinaFloat),
) -> [u8; 3] {
    let brightness = (1.0 - (hit.z - near) / (far - near)).clamp(0.0, 1.0);
    let v = (brightness * 255.0) as u8;
    [v, v, v]
}
