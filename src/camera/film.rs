use crate::{
    math::{PatinaFloat, PatinaPoint2f},
    spectrum::SampledSpectrum,
};

/// A pixel on the film, in raster space: column x and row y
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PixelCoord {
    pub x: u32,
    pub y: u32,
}

impl PixelCoord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// The continuous film (raster space) position at `offset`, in [0, 1)^2, inside this pixel.
    pub fn film_point(self, offset: PatinaPoint2f) -> PatinaPoint2f {
        debug_assert!((0.0..1.0).contains(&offset.x) && (0.0..1.0).contains(&offset.y));
        PatinaPoint2f::new(
            self.x as PatinaFloat + offset.x,
            self.y as PatinaFloat + offset.y,
        )
    }
}

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

    /// Every pixel of the film, row by row from the top left.
    pub fn pixels(&self) -> impl Iterator<Item = PixelCoord> + use<> {
        let (width, height) = (self.width as u32, self.height as u32);
        (0..height).flat_map(move |y| (0..width).map(move |x| PixelCoord::new(x, y)))
    }

    // The position of a pixel in `pixels`, stored row by row.
    fn index(&self, p: PixelCoord) -> usize {
        debug_assert!(
            (p.x as usize) < self.width && (p.y as usize) < self.height,
            "pixel {p:?} outside a {}x{} film",
            self.width,
            self.height
        );
        p.y as usize * self.width + p.x as usize
    }

    pub fn add_sample(
        &mut self,
        sample: SampledSpectrum,
        p_pixel: PixelCoord,
        weight: PatinaFloat,
    ) {
        debug_assert!(!sample.has_nan(), "NaN sample added to pixel {p_pixel:?}");

        let idx = self.index(p_pixel);
        let pixel = &mut self.pixels[idx];
        pixel.rgb_sum += sample * weight;
        pixel.weight_sum += weight;
    }
}

#[cfg(test)]
mod test_film {
    use approx::assert_relative_eq;

    use crate::{
        camera::film::{Film, PixelCoord},
        math::PatinaPoint2f,
        spectrum::SampledSpectrum,
    };

    #[test]
    fn test_add_sample_accumulates_weighted_sums() {
        let mut film = Film::new(4, 2);
        let p = PixelCoord::new(3, 1);
        film.add_sample(SampledSpectrum::new([1.0, 2.0, 3.0]), p, 1.0);
        film.add_sample(SampledSpectrum::new([3.0, 0.0, 1.0]), p, 3.0);

        let pixel = film.pixels[film.index(p)];
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
        film.add_sample(SampledSpectrum::constant(1.0), PixelCoord::new(1, 0), 1.0);

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
            PixelCoord::new(0, 0),
            1.0,
        );
    }

    #[test]
    fn test_pixels_are_visited_row_by_row() {
        let film = Film::new(3, 2);
        let pixels: Vec<PixelCoord> = film.pixels().collect();

        assert_eq!(pixels.len(), 6);
        assert_eq!(pixels[0], PixelCoord::new(0, 0));
        assert_eq!(pixels[2], PixelCoord::new(2, 0));
        assert_eq!(pixels[3], PixelCoord::new(0, 1));
        // Row by row is also the order the pixels are stored in.
        for (i, p) in pixels.iter().enumerate() {
            assert_eq!(film.index(*p), i);
        }
    }

    #[test]
    fn test_film_point_is_inside_the_pixel() {
        let p = PixelCoord::new(3, 7).film_point(PatinaPoint2f::new(0.25, 0.5));

        assert_eq!(p, PatinaPoint2f::new(3.25, 7.5));
    }
}
