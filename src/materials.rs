// Materials describe how a surface scatters light, and create the BSDF for a particular point on it.

use crate::{
    bsdf::BSDF, bxdfs::DiffuseBxDF, shapes::SurfaceInteraction, spectrum::SampledSpectrum,
};

/// The materials a primitive can have.
#[derive(Clone, Copy, Debug)]
pub enum Material {
    /// A Lambertian surface reflecting the fraction `reflectance` of incident light (each value in
    /// [0, 1]), equally in all directions.
    Diffuse { reflectance: SampledSpectrum },
}

impl Material {
    /// The BSDF at a point on a surface with this material.
    pub fn bsdf(&self, isect: &SurfaceInteraction) -> BSDF {
        match self {
            Material::Diffuse { reflectance } => BSDF::new(isect.n, DiffuseBxDF::new(*reflectance)),
        }
    }
}
