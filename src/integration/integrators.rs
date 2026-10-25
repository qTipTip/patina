use crate::{
    camera::{
        Camera, CameraSample,
        film::{Film, PixelCoord},
    },
    math::{PatinaRay3f, RenderSpace},
    sampling::traits::Sampler,
    spectrum::SampledSpectrum,
};

// TODO: Figure out where to put the sampler / light.
struct Light {}

trait Integrator {
    // Incident light
    fn li(&self, ray: &PatinaRay3f<RenderSpace>, sampler: &mut dyn Sampler) -> SampledSpectrum;
}

fn evaluate_pixel_sample(
    integrator: &impl Integrator,
    camera: &impl Camera,
    sampler: &mut impl Sampler,
    p_pixel: PixelCoord,
    film: &mut Film,
) {
    // Given a pixel (x, y), creates an offset in [0, 1), so we get an arbitrary point within the
    // pixel (so to speak).
    let p_film = p_pixel.film_point(sampler.get_pixel_2d());
    let p_lens = sampler.get_pixel_2d();
    let sample = CameraSample::new(p_film, p_lens, sampler.get_1d(), 1.0);

    // Generate a ray from the sample
    if let Some(ray) = camera.generate_ray(&sample) {
        // Compute the light contribution along the ray and add it to the film
        let l = integrator.li(&ray, sampler);
        film.add_sample(l, p_pixel, 1.0);
    }
}

struct RayIntegrator {
    camera: Box<dyn Camera>,
    sampler: Box<dyn Sampler>,
}
