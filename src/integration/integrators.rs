use crate::{
    camera::{
        Camera, CameraSample,
        film::{Film, PixelCoord},
    },
    math::{PatinaFloat, PatinaRay3f, RenderSpace},
    primitive::{Aggregate, PrimitiveIntersection},
    sampling::{independent_sampler::IndependentSampler, traits::Sampler},
    spectrum::SampledSpectrum,
};

pub trait Integrator {
    /// The scene`s primitives.
    fn aggregate(&self) -> &Aggregate;

    /// The radiance arriving at the ray's origin from the direction it points in.
    fn li(&self, ray: &PatinaRay3f<RenderSpace>, sampler: &mut dyn Sampler) -> SampledSpectrum;

    /// The closest hit along the ray in [0, t_max], with t_max = infinity if not given (pbrt's
    /// Integrator::Intersect).
    fn intersect(
        &self,
        ray: &PatinaRay3f<RenderSpace>,
        t_max: Option<PatinaFloat>,
    ) -> Option<PrimitiveIntersection<'_>> {
        self.aggregate().intersect(ray, t_max)
    }
}

/// Renders the scene onto the film, with `samples_per_pixel` camera samples for every pixel.
///
/// Each pixel has its own sampler, seeded from `seed` and the pixel's position.
pub fn render(
    integrator: &impl Integrator,
    camera: &impl Camera,
    film: &mut Film,
    samples_per_pixel: u32,
    seed: u64,
) {
    for p_pixel in film.pixels() {
        let pixel_seed = seed ^ ((p_pixel.y as u64) << 32 | p_pixel.x as u64);
        let mut sampler = IndependentSampler::new(samples_per_pixel, pixel_seed);
        for _ in 0..samples_per_pixel {
            evaluate_pixel_sample(integrator, camera, &mut sampler, p_pixel, film);
        }
    }
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
    let p_lens = sampler.get_2d();
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

#[cfg(test)]
mod test_render {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        camera::{OrthographicCamera, RenderingSpace, film::Film},
        integration::{integrators::render, randomwalk::RandomWalkIntegrator},
        materials::Material,
        math::{Bounds2f, PatinaPoint2f, PatinaVec3f, Transform},
        primitive::{Aggregate, Primitive},
        shapes::Sphere,
        spectrum::SampledSpectrum,
    };

    fn camera(width: u32, height: u32) -> OrthographicCamera {
        OrthographicCamera::new(
            &Transform::identity(),
            RenderingSpace::Camera,
            0.0,
            1.0,
            (width, height),
            &Bounds2f::new(PatinaPoint2f::new(-1.0, -1.0), PatinaPoint2f::new(1.0, 1.0)),
        )
    }

    #[test]
    fn test_empty_scene_renders_the_sky_everywhere() {
        let sky = SampledSpectrum::new([0.2, 0.5, 0.9]);
        let integrator = RandomWalkIntegrator::new(Aggregate::new(vec![]), sky, 3);
        let mut film = Film::new(4, 3);
        render(&integrator, &camera(4, 3), &mut film, 4, 1);

        for p in film.pixels() {
            assert_relative_eq!(film.pixel_value(p), sky);
        }
    }

    #[test]
    fn test_renders_are_deterministic() {
        let render_once = || {
            let sphere = Sphere::new(
                0.5,
                Transform::translate(&PatinaVec3f::new(0.0, 0.0, 3.0)),
                -0.5,
                0.5,
                2.0 * PI,
                false,
            );
            let aggregate = Aggregate::new(vec![Primitive::new(
                Box::new(sphere),
                Material::Diffuse {
                    reflectance: SampledSpectrum::constant(0.5),
                },
            )]);
            let integrator =
                RandomWalkIntegrator::new(aggregate, SampledSpectrum::constant(1.0), 3);
            let mut film = Film::new(8, 8);
            render(&integrator, &camera(8, 8), &mut film, 4, 42);
            film.pixels()
                .map(|p| film.pixel_value(p))
                .collect::<Vec<_>>()
        };

        assert_eq!(render_once(), render_once());
    }
}
