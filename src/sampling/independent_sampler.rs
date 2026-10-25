use rand::prelude::StdRng;
use rand::{RngExt, SeedableRng, distr};

use crate::math::{PatinaFloat, PatinaPoint2f};
use crate::sampling::traits::Sampler;

struct IndependentSampler {
    samples_per_pixel: u32,
    rng: StdRng,
    uniform: distr::Uniform<PatinaFloat>,
}

impl IndependentSampler {
    pub fn new(samples_per_pixel: u32, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let uniform = rand::distr::Uniform::new(0.0, 1.0).unwrap();
        Self {
            samples_per_pixel,
            rng,
            uniform,
        }
    }
}

impl Sampler for IndependentSampler {
    fn get_1d(&mut self) -> crate::math::PatinaFloat {
        self.rng.sample(self.uniform)
    }

    fn get_2d(&mut self) -> crate::math::PatinaPoint2f {
        PatinaPoint2f::new(self.rng.sample(self.uniform), self.rng.sample(self.uniform))
    }

    fn get_pixel_2d(&mut self) -> crate::math::PatinaPoint2f {
        self.get_2d()
    }
}
