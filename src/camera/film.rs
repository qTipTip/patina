use crate::{
    math::{PatinaFloat, PatinaPoint2i, points::PatinaPoint2u},
    spectrum::SampledSpectrum,
};

pub struct Film {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[derive(Clone, Copy)]
struct Pixel {
    rgb_sum: [PatinaFloat; 3],
    weight_sum: PatinaFloat,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![
                Pixel {
                    rgb_sum: [0.0, 0.0, 0.0],
                    weight_sum: 0.0
                };
                width * height
            ],
        }
    }

    pub fn add_sample(
        &mut self,
        sample: SampledSpectrum,
        p_pixel: PatinaPoint2u,
        weight: PatinaFloat,
    ) {
        debug_assert!(p_pixel.x >= 0 && (p_pixel.x as usize) < self.width);
        debug_assert!(p_pixel.y >= 0 && (p_pixel.y as usize) < self.height);

        let idx = p_pixel.y as usize * self.width + p_pixel.x as usize;
        self.pixels[idx].rgb_sum += sample.rgb;
        self.pixels[idx].weight_sum += sample.weight;
    }
}
