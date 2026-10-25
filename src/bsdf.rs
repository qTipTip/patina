// The BSDF at a point on a surface: a BxDF together with the local shading frame it works in. It
// takes and returns render-space directions, converting them to and from the BxDF's ShadingSpace.

use crate::{
    bxdfs::{BSDFSample, DiffuseBxDF},
    math::{Frame, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaVec3f, RenderSpace},
    spectrum::SampledSpectrum,
};

pub struct BSDF {
    bxdf: DiffuseBxDF,
    shading_frame: Frame<RenderSpace>,
}

impl BSDF {
    /// A BSDF for `bxdf` at a point with (unit) shading normal `ns`.
    pub fn new(ns: PatinaNorm3f<RenderSpace>, bxdf: DiffuseBxDF) -> Self {
        Self {
            bxdf,
            shading_frame: Frame::from_normal(ns),
        }
    }

    /// The BSDF value for light arriving from `wi` and leaving along `wo`.
    pub fn f(&self, wo: PatinaVec3f<RenderSpace>, wi: PatinaVec3f<RenderSpace>) -> SampledSpectrum {
        let (wo, wi) = (
            self.shading_frame.to_local(wo),
            self.shading_frame.to_local(wi),
        );
        // A direction exactly in the tangent plane has no defined side of the surface.
        if wo.z == 0.0 {
            return SampledSpectrum::default();
        }
        self.bxdf.f(wo, wi)
    }

    /// Samples an incident direction for light leaving along `wo`. `u` is a uniform sample in
    /// [0, 1)^2.
    pub fn sample_f(
        &self,
        wo: PatinaVec3f<RenderSpace>,
        u: PatinaPoint2f,
    ) -> Option<BSDFSample<RenderSpace>> {
        let wo = self.shading_frame.to_local(wo);
        if wo.z == 0.0 {
            return None;
        }
        let sample = self.bxdf.sample_f(wo, u)?;
        if sample.f.is_black() || sample.pdf == 0.0 || sample.wi.z == 0.0 {
            return None;
        }
        Some(BSDFSample {
            f: sample.f,
            wi: self.shading_frame.from_local(sample.wi),
            pdf: sample.pdf,
        })
    }

    /// The density with which sample_f returns `wi`, given `wo`.
    pub fn pdf(&self, wo: PatinaVec3f<RenderSpace>, wi: PatinaVec3f<RenderSpace>) -> PatinaFloat {
        let (wo, wi) = (
            self.shading_frame.to_local(wo),
            self.shading_frame.to_local(wi),
        );
        if wo.z == 0.0 {
            return 0.0;
        }
        self.bxdf.pdf(wo, wi)
    }
}

#[cfg(test)]
mod test_bsdf {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        bsdf::BSDF,
        bxdfs::DiffuseBxDF,
        math::{PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaVec3f, RenderSpace},
        spectrum::SampledSpectrum,
    };

    fn reflectance() -> SampledSpectrum {
        SampledSpectrum::constant(0.5)
    }

    // A surface whose normal points along (1, 1, 0), so its local frame is tilted relative to
    // render space.
    fn tilted_bsdf() -> (BSDF, PatinaVec3f<RenderSpace>) {
        let n = PatinaVec3f::new(1.0, 1.0, 0.0).normalize();
        let bsdf = BSDF::new(
            PatinaNorm3f::new(n.x, n.y, n.z),
            DiffuseBxDF::new(reflectance()),
        );
        (bsdf, n)
    }

    #[test]
    fn test_f_uses_the_side_of_the_normal() {
        let (bsdf, n) = tilted_bsdf();
        let wo = PatinaVec3f::new(1.0, 0.2, 0.3).normalize();

        // Same side of the tilted surface as wo, though it points down in render space.
        let wi_same = PatinaVec3f::new(0.2, 1.0, -0.8).normalize();
        assert!(wi_same.dot(&n) > 0.0);
        assert_relative_eq!(bsdf.f(wo, wi_same), reflectance() / PI);

        // The other side of the surface.
        let wi_other = PatinaVec3f::new(-1.0, -0.2, 0.5).normalize();
        assert!(bsdf.f(wo, wi_other).is_black());
    }

    #[test]
    fn test_samples_are_in_render_space_on_the_side_of_wo() {
        let (bsdf, n) = tilted_bsdf();
        let wo = PatinaVec3f::new(1.0, 0.2, 0.3).normalize();
        for i in 0..8 {
            for j in 0..8 {
                let u = PatinaPoint2f::new(
                    (i as PatinaFloat + 0.5) / 8.0,
                    (j as PatinaFloat + 0.5) / 8.0,
                );
                let s = bsdf
                    .sample_f(wo, u)
                    .expect("diffuse sampling always succeeds");

                assert_relative_eq!(s.wi.len(), 1.0, epsilon = 1e-12);
                assert!(s.wi.dot(&n) > 0.0);
                assert_relative_eq!(s.pdf, bsdf.pdf(wo, s.wi), epsilon = 1e-12);
                // The sample's weight is the reflectance, as for the BxDF alone.
                assert_relative_eq!(
                    s.f * s.wi.dot(&n).abs() / s.pdf,
                    reflectance(),
                    epsilon = 1e-12
                );
            }
        }
    }
}
