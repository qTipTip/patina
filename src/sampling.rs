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

/// Maps a uniform sample `u` in [0, 1)^2 to a point distributed uniformly over the unit disk.
pub fn sample_uniform_disk_concentric(u: PatinaPoint2f) -> PatinaPoint2f {
    // Map u to [-1, 1]^2; its center maps to the center of the disk.
    let (ox, oy) = (2.0 * u.x - 1.0, 2.0 * u.y - 1.0);
    if ox == 0.0 && oy == 0.0 {
        return PatinaPoint2f::zero();
    }

    // Use the larger coordinate as the radius, and the ratio of the two to place the angle within
    // that square's side.
    let (r, theta) = if ox.abs() > oy.abs() {
        (ox, (PI / 4.0) * (oy / ox))
    } else {
        (oy, PI / 2.0 - (PI / 4.0) * (ox / oy))
    };
    PatinaPoint2f::new(r * theta.cos(), r * theta.sin())
}

/// Maps a uniform sample `u` in [0, 1)^2 to a direction in the upper (z >= 0) hemisphere, with a
/// density proportional to cos(theta) = z.
pub fn sample_cosine_hemisphere<S>(u: PatinaPoint2f) -> PatinaVec3f<S> {
    let d = sample_uniform_disk_concentric(u);
    let z = (1.0 - d.x * d.x - d.y * d.y).max(0.0).sqrt();
    PatinaVec3f::new(d.x, d.y, z)
}

/// The probability density of sample_cosine_hemisphere, for a direction at angle theta to the
/// z-axis.
pub fn cosine_hemisphere_pdf(cos_theta: PatinaFloat) -> PatinaFloat {
    cos_theta / PI
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

#[cfg(test)]
mod test_cosine_hemisphere {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        math::{PatinaFloat, PatinaPoint2f, PatinaVec3f, WorldSpace},
        sampling::{
            cosine_hemisphere_pdf, sample_cosine_hemisphere, sample_uniform_disk_concentric,
        },
    };

    // The centers of an n x n grid of cells in [0, 1)^2: a deterministic stand-in for many random
    // samples.
    fn grid(n: usize) -> impl Iterator<Item = PatinaPoint2f> {
        let cell = move |i: usize| (i as PatinaFloat + 0.5) / n as PatinaFloat;
        (0..n).flat_map(move |i| (0..n).map(move |j| PatinaPoint2f::new(cell(i), cell(j))))
    }

    #[test]
    fn test_disk_samples_are_inside_the_unit_disk() {
        assert_eq!(
            sample_uniform_disk_concentric(PatinaPoint2f::new(0.5, 0.5)),
            PatinaPoint2f::zero()
        );
        for u in grid(64) {
            let d = sample_uniform_disk_concentric(u);
            assert!(d.x * d.x + d.y * d.y <= 1.0 + 1e-12);
        }
    }

    #[test]
    fn test_disk_samples_are_uniform() {
        // For a uniform distribution over the unit disk, the mean is the center, and the mean
        // squared distance from the center is 1/2.
        let samples: Vec<_> = grid(200).map(sample_uniform_disk_concentric).collect();
        let n = samples.len() as PatinaFloat;
        let mean_x = samples.iter().map(|d| d.x).sum::<PatinaFloat>() / n;
        let mean_y = samples.iter().map(|d| d.y).sum::<PatinaFloat>() / n;
        let mean_r2 = samples
            .iter()
            .map(|d| d.x * d.x + d.y * d.y)
            .sum::<PatinaFloat>()
            / n;

        assert_relative_eq!(mean_x, 0.0, epsilon = 1e-3);
        assert_relative_eq!(mean_y, 0.0, epsilon = 1e-3);
        assert_relative_eq!(mean_r2, 0.5, epsilon = 1e-3);
    }

    #[test]
    fn test_hemisphere_samples_are_unit_vectors_facing_up() {
        for u in grid(64) {
            let w: PatinaVec3f<WorldSpace> = sample_cosine_hemisphere(u);
            assert_relative_eq!(w.len(), 1.0, epsilon = 1e-12);
            assert!(w.z >= 0.0);
        }
    }

    #[test]
    fn test_hemisphere_samples_are_cosine_distributed() {
        // With density cos(theta) / pi, the mean of cos(theta) = z is 2/3 (it would be 1/2 for
        // uniform hemisphere samples).
        let samples: Vec<PatinaVec3f<WorldSpace>> =
            grid(200).map(sample_cosine_hemisphere).collect();
        let mean_z =
            samples.iter().map(|w| w.z).sum::<PatinaFloat>() / samples.len() as PatinaFloat;

        assert_relative_eq!(mean_z, 2.0 / 3.0, epsilon = 1e-3);
    }

    #[test]
    fn test_pdf_integrates_to_one() {
        // Integrating cos(theta) / pi over the hemisphere, with d(omega) = sin(theta) dtheta dphi.
        let n = 1000;
        let d_theta = (PI / 2.0) / n as PatinaFloat;
        let integral: PatinaFloat = (0..n)
            .map(|i| {
                let theta = (i as PatinaFloat + 0.5) * d_theta;
                cosine_hemisphere_pdf(theta.cos()) * theta.sin() * d_theta * 2.0 * PI
            })
            .sum();

        assert_relative_eq!(integral, 1.0, epsilon = 1e-6);
    }
}
