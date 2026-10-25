#![allow(dead_code)]

use std::f64::consts::PI;

use crate::math::{PatinaFloat, PatinaPoint2f, PatinaVec3f};
pub mod bilinear;
pub mod independent_sampler;
pub mod linear;
pub mod traits;

/// Maps a uniform sample `u` in [0, 1)^2 to a direction distributed uniformly over the unit sphere
///  The resulting vecotr is in whichever space the caller uses it in.
pub fn sample_uniform_sphere<S>(u: PatinaPoint2f) -> PatinaVec3f<S> {
    let z = 1.0 - 2.0 * u.x;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;
    PatinaVec3f::new(r * phi.cos(), r * phi.sin(), z)
}

/// The probability density of sample_uniform_sphere, with respect to solid angle
pub fn uniform_sphere_pdf() -> PatinaFloat {
    1.0 / (4.0 * PI)
}

#[cfg(test)]
mod test_uniform_sphere {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        math::{PatinaFloat, PatinaPoint2f, PatinaVec3f, WorldSpace},
        sampling::{sample_uniform_sphere, uniform_sphere_pdf},
    };

    fn sample(u0: PatinaFloat, u1: PatinaFloat) -> PatinaVec3f<WorldSpace> {
        sample_uniform_sphere(PatinaPoint2f::new(u0, u1))
    }

    // Samples on an n x n grid of u values, at the centers of the grid cells.
    fn grid_samples(n: usize) -> Vec<PatinaVec3f<WorldSpace>> {
        let cell = |i: usize| (i as PatinaFloat + 0.5) / n as PatinaFloat;
        (0..n)
            .flat_map(|i| (0..n).map(move |j| sample(cell(i), cell(j))))
            .collect()
    }

    #[test]
    fn test_known_directions() {
        // u[0] = 0 is the top of the sphere, u[0] = 0.5 the equator.
        assert_relative_eq!(sample(0.0, 0.3), PatinaVec3f::new(0.0, 0.0, 1.0));
        assert_relative_eq!(sample(0.5, 0.0), PatinaVec3f::new(1.0, 0.0, 0.0));
        assert_relative_eq!(sample(0.5, 0.25), PatinaVec3f::new(0.0, 1.0, 0.0));
        assert_relative_eq!(sample(0.5, 0.5), PatinaVec3f::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_samples_are_unit_vectors() {
        for v in grid_samples(32) {
            assert_relative_eq!(v.len(), 1.0, epsilon = 1e-12);
        }
    }

    #[test]
    fn test_samples_are_uniform() {
        // For a uniform distribution over the sphere, every direction is balanced by its opposite,
        // so the mean is the zero vector. Also, by symmetry x^2, y^2 and z^2 each average to 1/3.
        let samples = grid_samples(200);
        let n = samples.len() as PatinaFloat;
        let mean = samples
            .iter()
            .fold(PatinaVec3f::<WorldSpace>::zero(), |acc, v| acc + *v)
            / n;
        let mean_square = |c: fn(&PatinaVec3f<WorldSpace>) -> PatinaFloat| {
            samples.iter().map(|v| c(v) * c(v)).sum::<PatinaFloat>() / n
        };

        assert_relative_eq!(mean, PatinaVec3f::zero(), epsilon = 1e-3);
        assert_relative_eq!(mean_square(|v| v.x), 1.0 / 3.0, epsilon = 1e-3);
        assert_relative_eq!(mean_square(|v| v.y), 1.0 / 3.0, epsilon = 1e-3);
        assert_relative_eq!(mean_square(|v| v.z), 1.0 / 3.0, epsilon = 1e-3);
    }

    #[test]
    fn test_pdf_integrates_to_one() {
        // A density over solid angle integrates to 1 over the sphere's 4 pi steradians.
        assert_relative_eq!(uniform_sphere_pdf() * 4.0 * PI, 1.0);
    }
}
