// Primitives pair a shape's geometry with a material, and an aggregate holds the primitives of a
// scene, so the integrator can ask which surface a ray hits first and which materials it's made of.

use crate::{
    materials::Material,
    math::{PatinaFloat, PatinaRay3f, RenderSpace},
    shapes::{Shape, ShapeIntersection},
};

/// A shape with a material.
pub struct Primitive {
    pub shape: Box<dyn Shape>,
    pub material: Material,
}

impl Primitive {
    pub fn new(shape: Box<dyn Shape>, material: Material) -> Self {
        Self { shape, material }
    }
}

/// A ray-primitive hit. the geometry of the hit, and the material of the primitive that was hit.
pub struct PrimitiveIntersection<'a> {
    pub intersection: ShapeIntersection,
    pub material: &'a Material,
}

/// All the primitives of a scene. For now a plain list that tests every
/// primitive. TODO: Implement an acceleration structure such as a BVH that can replace it behind the same interface.
pub struct Aggregate {
    primitives: Vec<Primitive>,
}

impl Aggregate {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }

    /// The closest hit along the ray in [0, t_max], with t_max = infinity if not given. Each hit
    /// lowers t_max, so primitives behind the closest one so far are rejected.
    pub fn intersect(
        &self,
        ray: &PatinaRay3f<RenderSpace>,
        t_max: Option<PatinaFloat>,
    ) -> Option<PrimitiveIntersection<'_>> {
        let mut t_max = t_max.unwrap_or(PatinaFloat::INFINITY);
        let mut closest = None;
        for primitive in &self.primitives {
            if let Some(intersection) = primitive.shape.intersect(ray, Some(t_max)) {
                t_max = intersection.t_hit;
                closest = Some(PrimitiveIntersection {
                    intersection,
                    material: &primitive.material,
                });
            }
        }
        closest
    }
}

#[cfg(test)]
mod test_aggregate {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        materials::Material,
        math::{PatinaPoint3f, PatinaRay3f, PatinaVec3f, RenderSpace, Transform},
        primitive::{Aggregate, Primitive},
        shapes::Sphere,
        spectrum::SampledSpectrum,
    };

    // A unit sphere at (0, 0, z) with a diffuse material of the given (gray) reflectance.
    fn sphere_at_z(z: f64, reflectance: f64) -> Primitive {
        let sphere = Sphere::new(
            1.0,
            Transform::translate(&PatinaVec3f::new(0.0, 0.0, z)),
            -1.0,
            1.0,
            2.0 * PI,
            false,
        );
        Primitive::new(
            Box::new(sphere),
            Material::Diffuse {
                reflectance: SampledSpectrum::constant(reflectance),
            },
        )
    }

    // A ray from the origin along +z.
    fn ray_along_z() -> PatinaRay3f<RenderSpace> {
        PatinaRay3f::new(PatinaPoint3f::zero(), PatinaVec3f::new(0.0, 0.0, 1.0), 0.0)
    }

    fn reflectance_of(material: &Material) -> SampledSpectrum {
        match material {
            Material::Diffuse { reflectance } => *reflectance,
        }
    }

    #[test]
    fn test_closest_hit_wins_whatever_the_order() {
        // The far sphere is listed first, so the search must not stop at the first hit.
        let aggregate = Aggregate::new(vec![sphere_at_z(10.0, 0.2), sphere_at_z(5.0, 0.8)]);
        let hit = aggregate
            .intersect(&ray_along_z(), None)
            .expect("ray should hit both spheres");

        assert_relative_eq!(hit.intersection.t_hit, 4.0);
        assert_relative_eq!(reflectance_of(hit.material), SampledSpectrum::constant(0.8));
    }

    #[test]
    fn test_miss() {
        let aggregate = Aggregate::new(vec![sphere_at_z(5.0, 0.8)]);
        let ray = PatinaRay3f::new(PatinaPoint3f::zero(), PatinaVec3f::new(0.0, 1.0, 0.0), 0.0);

        assert!(aggregate.intersect(&ray, None).is_none());
    }

    #[test]
    fn test_empty_aggregate() {
        assert!(
            Aggregate::new(vec![])
                .intersect(&ray_along_z(), None)
                .is_none()
        );
    }

    #[test]
    fn test_t_max_limits_the_search() {
        let aggregate = Aggregate::new(vec![sphere_at_z(10.0, 0.2), sphere_at_z(5.0, 0.8)]);

        // The near sphere is entered at t = 4.
        assert!(aggregate.intersect(&ray_along_z(), Some(3.0)).is_none());
        let hit = aggregate
            .intersect(&ray_along_z(), Some(4.5))
            .expect("the near sphere is within t_max");
        assert_relative_eq!(hit.intersection.t_hit, 4.0);
    }

    #[test]
    fn test_material_bsdf_at_the_hit() {
        // The hit is on the near side of the sphere, so its normal points back along the ray.
        let aggregate = Aggregate::new(vec![sphere_at_z(5.0, 0.8)]);
        let hit = aggregate
            .intersect(&ray_along_z(), None)
            .expect("ray should hit");
        let bsdf = hit.material.bsdf(&hit.intersection.interaction);
        let wo = hit.intersection.interaction.wo;

        // Light from the camera's side is reflected; light from behind the surface is not.
        let towards_camera = PatinaVec3f::new(0.3, 0.1, -1.0).normalize();
        assert_relative_eq!(
            bsdf.f(wo, towards_camera),
            SampledSpectrum::constant(0.8 / PI)
        );
        assert!(bsdf.f(wo, PatinaVec3f::new(0.0, 0.0, 1.0)).is_black());
    }
}
