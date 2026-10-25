use crate::{
    camera::{Camera, RenderingSpace},
    math::PatinaRay3f,
    sampling::traits::Sampler,
};

// TODO: Figure out where to put the sampler / light / sampledspectrum.
struct Light {}
struct SampledSpectrum {}

trait Integrator {
    // Incident light
    fn li(&self, ray: &PatinaRay3f<RenderingSpace>) -> SampledSpectrum;
    // Given a pixel, generate a ray using the camera and the sampler, calls the li-method, then
    // stores the contribution to the camera film.
    // fn evaluate_pixel_sample(&self) -> {
    // }
}

struct RayIntegrator {
    camera: Box<dyn Camera>,
    sampler: Box<dyn Sampler>,
    lights: Vec<Light>,
}
