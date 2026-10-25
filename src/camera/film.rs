use crate::{
    integration::integrators::SampledSpectrum,
    math::{PatinaFloat, PatinaPoint2i},
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

    pub fn add_sample(
        &mut self,
        sample: SampledSpectrum,
        p_pixel: PatinaPoint2i,
        weight: PatinaFloat,
    ) {
        todo!();
    }
}
