use rand::SeedableRng;
use rand::prelude::StdRng;

struct IndependentSampler {
    samples_per_pixel: u32,
    rng: StdRng,
}

impl IndependentSampler {
    pub fn new(samples_per_pixel: u32, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        Self {
            samples_per_pixel,
            rng,
        }
    }
}
