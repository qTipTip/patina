use crate::{
    integration::integrators::Integrator,
    math::{PatinaRay3f, PatinaVec3f, RenderSpace},
    primitive::Aggregate,
    sampling::{sample_uniform_sphere, traits::Sampler, uniform_sphere_pdf},
    spectrum::SampledSpectrum,
};

/// This is the simplest integrator described in the PBRT book. At each hit, you continue the path
/// in a uniformly random direction on the sphere, and add up the light it eventually reaches. For
/// now, the only light source is the sky, which has a constant radiance.
pub struct RandomWalkIntegrator {
    aggregate: Aggregate,
    /// The radiance of the uniform infinite light, seen by every ray that leaves the scene.
    sky: SampledSpectrum,
    /// The number of bounces after which a path is cut off.
    max_depth: u32,
}

impl RandomWalkIntegrator {
    pub fn new(aggregate: Aggregate, sky: SampledSpectrum, max_depth: u32) -> Self {
        Self {
            aggregate,
            sky,
            max_depth,
        }
    }

    // `beta` is the path's throughput, the product of f * |cos| / pdf over the bounces so far,
    // which weighs the light found at the end of the path.
    fn li_random_walk(
        &self,
        ray: PatinaRay3f<RenderSpace>,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        let mut l = SampledSpectrum::default();
        let mut beta = SampledSpectrum::constant(1.0);
        let mut ray = ray;
        let mut depth = 0;

        loop {
            let Some(hit) = self.intersect(&ray, None) else {
                // The path escaped the scene and sees the sky
                l += beta * self.sky;
                break;
            };
            // surfaces do not emit light yet, so a hit adds nothing by itself
            if depth == self.max_depth {
                break;
            }
            depth += 1;

            // Continue in a uniformly random direction, weighted by how much light the surface
            // sends from that direction towards wo.
            let isect = &hit.intersection.interaction;
            let bsdf = hit.material.bsdf(isect);
            let wi: PatinaVec3f<RenderSpace> = sample_uniform_sphere(sampler.get_2d());
            let n = PatinaVec3f::new(isect.n.x, isect.n.y, isect.n.z);
            let f_cos = bsdf.f(isect.wo, wi) * wi.dot(&n).abs();
            if f_cos.is_black() {
                break;
            }

            beta *= f_cos / uniform_sphere_pdf();
            ray = isect.spawn_ray(wi);
        }
        l
    }
}

impl Integrator for RandomWalkIntegrator {
    fn aggregate(&self) -> &Aggregate {
        &self.aggregate
    }

    fn li(&self, ray: &PatinaRay3f<RenderSpace>, sampler: &mut dyn Sampler) -> SampledSpectrum {
        self.li_random_walk(*ray, sampler)
    }
}

#[cfg(test)]
mod test_random_walk {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        integration::{integrators::Integrator, randomwalk::RandomWalkIntegrator},
        materials::Material,
        math::{PatinaFloat, PatinaPoint3f, PatinaRay3f, PatinaVec3f, RenderSpace, Transform},
        primitive::{Aggregate, Primitive},
        sampling::independent_sampler::IndependentSampler,
        shapes::Sphere,
        spectrum::SampledSpectrum,
    };

    // A diffuse unit sphere at (0, 0, 5) with the given reflectance, under a uniform sky.
    fn sphere_under_sky(
        reflectance: SampledSpectrum,
        sky: SampledSpectrum,
        max_depth: u32,
    ) -> RandomWalkIntegrator {
        let sphere = Sphere::new(
            1.0,
            Transform::translate(&PatinaVec3f::new(0.0, 0.0, 5.0)),
            -1.0,
            1.0,
            2.0 * PI,
            false,
        );
        let aggregate = Aggregate::new(vec![Primitive::new(
            Box::new(sphere),
            Material::Diffuse { reflectance },
        )]);
        RandomWalkIntegrator::new(aggregate, sky, max_depth)
    }

    fn ray_towards(d: PatinaVec3f<RenderSpace>) -> PatinaRay3f<RenderSpace> {
        PatinaRay3f::new(PatinaPoint3f::zero(), d.normalize(), 0.0)
    }

    // The average of n samples of li along `ray`.
    fn average_li(
        integrator: &RandomWalkIntegrator,
        ray: &PatinaRay3f<RenderSpace>,
        n: usize,
    ) -> SampledSpectrum {
        let mut sampler = IndependentSampler::new(n as u32, 7);
        let mut sum = SampledSpectrum::default();
        for _ in 0..n {
            sum += integrator.li(ray, &mut sampler);
        }
        sum / n as PatinaFloat
    }

    #[test]
    fn test_escaping_ray_sees_the_sky() {
        let sky = SampledSpectrum::new([0.2, 0.4, 0.9]);
        let integrator = sphere_under_sky(SampledSpectrum::constant(0.5), sky, 5);
        let ray = ray_towards(PatinaVec3f::new(0.0, 1.0, 0.0));

        assert_relative_eq!(average_li(&integrator, &ray, 10), sky);
    }

    #[test]
    fn test_furnace() {
        // A convex object can't see itself, so every bounce off it escapes to the sky. Its pixels
        // therefore converge to exactly reflectance * sky
        let reflectance = SampledSpectrum::new([0.8, 0.5, 0.2]);
        let sky = SampledSpectrum::constant(2.0);
        let integrator = sphere_under_sky(reflectance, sky, 5);

        for ray in [
            ray_towards(PatinaVec3f::new(0.0, 0.0, 1.0)),
            ray_towards(PatinaVec3f::new(0.1, -0.12, 1.0)),
        ] {
            let estimate = average_li(&integrator, &ray, 200_000);
            assert_relative_eq!(estimate, reflectance * sky, epsilon = 0.02);
        }
    }

    #[test]
    fn test_max_depth_zero_sees_no_light_from_surfaces() {
        // Without bounces, a path that hits the sphere ends there, and surfaces don't emit light.
        let integrator = sphere_under_sky(
            SampledSpectrum::constant(0.5),
            SampledSpectrum::constant(1.0),
            0,
        );
        let ray = ray_towards(PatinaVec3f::new(0.0, 0.0, 1.0));

        assert!(average_li(&integrator, &ray, 100).is_black());
    }
}
