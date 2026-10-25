use crate::{
    math::{PatinaFloat, points::PatinaPoint2u},
    spectrum::SampledSpectrum,
};

pub struct Film {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

// A pixel accumulates the weighted sum of its samples and the sum of their weights. We do it like
// this so we don't have to keep a running average.
#[derive(Clone, Copy, Default)]
struct Pixel {
    rgb_sum: SampledSpectrum,
    weight_sum: PatinaFloat,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    pub fn add_sample(
        &mut self,
        sample: SampledSpectrum,
        p_pixel: PatinaPoint2u,
        weight: PatinaFloat,
    ) {
        debug_assert!((p_pixel.x as usize) < self.width && (p_pixel.y as usize) < self.height);
        debug_assert!(!sample.has_nan(), "NaN sample added to pixel {p_pixel:?}");

        let pixel = &mut self.pixels[p_pixel.y as usize * self.width + p_pixel.x as usize];
        pixel.rgb_sum += sample * weight;
        pixel.weight_sum += weight;
    }
}

#[cfg(test)]
mod test_film {
    use approx::assert_relative_eq;

    use crate::{camera::film::Film, math::points::PatinaPoint2u, spectrum::SampledSpectrum};

    #[test]
    fn test_add_sample_accumulates_weighted_sums() {
        let mut film = Film::new(4, 2);
        let p = PatinaPoint2u::new(3, 1);
        film.add_sample(SampledSpectrum::new([1.0, 2.0, 3.0]), p, 1.0);
        film.add_sample(SampledSpectrum::new([3.0, 0.0, 1.0]), p, 3.0);

        let pixel = film.pixels[7];
        assert_relative_eq!(pixel.rgb_sum, SampledSpectrum::new([10.0, 2.0, 6.0]));
        assert_relative_eq!(pixel.weight_sum, 4.0);
        // The weighted average leans towards the sample with the larger weight.
        assert_relative_eq!(
            pixel.rgb_sum / pixel.weight_sum,
            SampledSpectrum::new([2.5, 0.5, 1.5])
        );
    }

    #[test]
    fn test_add_sample_only_touches_its_pixel() {
        let mut film = Film::new(4, 2);
        film.add_sample(
            SampledSpectrum::constant(1.0),
            PatinaPoint2u::new(1, 0),
            1.0,
        );

        for (i, pixel) in film.pixels.iter().enumerate() {
            let expected = if i == 1 { 1.0 } else { 0.0 };
            assert_relative_eq!(pixel.weight_sum, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_sample_rejects_nan() {
        let mut film = Film::new(1, 1);
        film.add_sample(
            SampledSpectrum::new([f64::NAN, 0.0, 0.0]),
            PatinaPoint2u::new(0, 0),
            1.0,
        );
    }
}
