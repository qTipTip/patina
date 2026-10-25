// how a surface scatters light, in the local shading frame (ShadingSpace), where the shading
// normal is +z.

use std::f64::consts::PI;

use crate::{
    math::{PatinaFloat, PatinaPoint2f, PatinaVec3f, ShadingSpace},
    sampling::{cosine_hemisphere_pdf, sample_cosine_hemisphere},
    spectrum::SampledSpectrum,
};

/// A direction sampled from a BxDF or BSDF, in space `S`: the BxDF's value for it, and the
/// probability density (with respect to solid angle) of having sampled it.
#[derive(Clone, Copy, Debug)]
pub struct BSDFSample<S> {
    pub f: SampledSpectrum,
    pub wi: PatinaVec3f<S>,
    pub pdf: PatinaFloat,
}

/// Whether two local directions are on the same side of the surface.
pub fn same_hemisphere(w: PatinaVec3f<ShadingSpace>, wp: PatinaVec3f<ShadingSpace>) -> bool {
    w.z * wp.z > 0.0
}

/// |cos(theta)| of a local direction, theta is its angle to the normal.
pub fn abs_cos_theta(w: PatinaVec3f<ShadingSpace>) -> PatinaFloat {
    w.z.abs()
}

/// Lambertian reflection: light is scattered equally in all directions of the hemisphere it arrived
/// from, so the BRDF is a constant, R / pi.
#[derive(Clone, Copy, Debug)]
pub struct DiffuseBxDF {
    r: SampledSpectrum,
}

impl DiffuseBxDF {
    /// A diffuse BxDF with reflectance `r`, the fraction of light reflected (each value in [0, 1]).
    pub fn new(r: SampledSpectrum) -> Self {
        Self { r }
    }

    /// The BRDF value for light arriving from `wi` and leaving along `wo`. Zero if they are on
    /// opposite sides of the surface, since a diffuse surface only reflects.
    pub fn f(
        &self,
        wo: PatinaVec3f<ShadingSpace>,
        wi: PatinaVec3f<ShadingSpace>,
    ) -> SampledSpectrum {
        if !same_hemisphere(wo, wi) {
            return SampledSpectrum::default();
        }
        self.r / PI
    }

    /// Samples an incident direction with density proportional to cos(theta), on the same side of
    /// the surface as `wo`. `u` is a uniform sample in [0, 1)^2.
    pub fn sample_f(
        &self,
        wo: PatinaVec3f<ShadingSpace>,
        u: PatinaPoint2f,
    ) -> Option<BSDFSample<ShadingSpace>> {
        let mut wi: PatinaVec3f<ShadingSpace> = sample_cosine_hemisphere(u);
        // Cosine sampling gives the upper hemisphere; flip to the side wo is on.
        if wo.z < 0.0 {
            wi.z = -wi.z;
        }
        let pdf = cosine_hemisphere_pdf(abs_cos_theta(wi));
        if pdf == 0.0 {
            return None;
        }
        Some(BSDFSample {
            f: self.r / PI,
            wi,
            pdf,
        })
    }

    /// The density with which sample_f returns `wi`, given `wo`.
    pub fn pdf(&self, wo: PatinaVec3f<ShadingSpace>, wi: PatinaVec3f<ShadingSpace>) -> PatinaFloat {
        if !same_hemisphere(wo, wi) {
            return 0.0;
        }
        cosine_hemisphere_pdf(abs_cos_theta(wi))
    }
}

#[cfg(test)]
mod test_diffuse_bxdf {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        bxdfs::{DiffuseBxDF, abs_cos_theta, same_hemisphere},
        math::{PatinaFloat, PatinaPoint2f, PatinaVec3f, ShadingSpace},
        spectrum::SampledSpectrum,
    };

    fn reflectance() -> SampledSpectrum {
        SampledSpectrum::new([0.8, 0.5, 0.2])
    }

    fn dir(x: PatinaFloat, y: PatinaFloat, z: PatinaFloat) -> PatinaVec3f<ShadingSpace> {
        PatinaVec3f::new(x, y, z).normalize()
    }

    // The centers of an n x n grid of cells in [0, 1)^2.
    fn grid(n: usize) -> impl Iterator<Item = PatinaPoint2f> {
        let cell = move |i: usize| (i as PatinaFloat + 0.5) / n as PatinaFloat;
        (0..n).flat_map(move |i| (0..n).map(move |j| PatinaPoint2f::new(cell(i), cell(j))))
    }

    #[test]
    fn test_f_is_constant_on_the_same_side() {
        let bxdf = DiffuseBxDF::new(reflectance());
        let wo = dir(0.3, -0.2, 1.0);

        assert_relative_eq!(bxdf.f(wo, dir(0.0, 0.0, 1.0)), reflectance() / PI);
        assert_relative_eq!(bxdf.f(wo, dir(-0.9, 0.4, 0.1)), reflectance() / PI);
        // Reciprocity: swapping the directions doesn't change the value.
        assert_relative_eq!(
            bxdf.f(wo, dir(1.0, 1.0, 1.0)),
            bxdf.f(dir(1.0, 1.0, 1.0), wo)
        );
    }

    #[test]
    fn test_f_is_zero_across_the_surface() {
        let bxdf = DiffuseBxDF::new(reflectance());

        assert!(bxdf.f(dir(0.0, 0.0, 1.0), dir(0.0, 0.0, -1.0)).is_black());
        assert!(bxdf.f(dir(0.2, 0.0, -1.0), dir(0.0, 0.3, 1.0)).is_black());
    }

    #[test]
    fn test_samples_are_on_the_side_of_wo() {
        let bxdf = DiffuseBxDF::new(reflectance());
        for wo in [dir(0.3, 0.1, 1.0), dir(0.3, 0.1, -1.0)] {
            for u in grid(16) {
                let s = bxdf
                    .sample_f(wo, u)
                    .expect("diffuse sampling always succeeds");
                assert!(same_hemisphere(wo, s.wi));
                assert_relative_eq!(s.pdf, bxdf.pdf(wo, s.wi));
            }
        }
    }

    #[test]
    fn test_sample_weight_is_the_reflectance() {
        // The Monte Carlo weight of a sample is f * |cos(theta)| / pdf. Cosine sampling makes the pdf
        // proportional to f * |cos(theta)|, so the weight is exactly the reflectance for every
        // sample.
        let bxdf = DiffuseBxDF::new(reflectance());
        let wo = dir(-0.4, 0.2, 1.0);
        for u in grid(16) {
            let s = bxdf
                .sample_f(wo, u)
                .expect("diffuse sampling always succeeds");
            assert_relative_eq!(
                s.f * abs_cos_theta(s.wi) / s.pdf,
                reflectance(),
                epsilon = 1e-12
            );
        }
    }

    #[test]
    fn test_pdf_is_zero_across_the_surface() {
        let bxdf = DiffuseBxDF::new(reflectance());

        assert_eq!(bxdf.pdf(dir(0.0, 0.0, 1.0), dir(0.0, 0.0, -1.0)), 0.0);
    }
}
