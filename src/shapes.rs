#![allow(dead_code)]
pub(crate) mod disk;
pub(crate) mod sphere;

pub use disk::Disk;
pub use sphere::Sphere;

use crate::math::{
    Bounds3f, ObjectSpace, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f, PatinaRay3f,
    PatinaVec3f, RenderSpace,
};

/// The local geometry at a ray-surface hit, in render space. A minimal version of pbrt's
/// SurfaceInteraction
pub struct SurfaceInteraction {
    /// The hit point.
    pub p: PatinaPoint3f<RenderSpace>,
    /// The geometric surface normal, normalized. Points outwards unless the shape's orientation
    /// is reversed.
    pub n: PatinaNorm3f<RenderSpace>,
    /// The shape's (u, v) parameterization of the hit point, each in [0, 1].
    pub uv: PatinaPoint2f,
    /// The normalized direction back along the ray, towards its origin.
    pub wo: PatinaVec3f<RenderSpace>,
    /// The time of the ray that produced the hit.
    pub time: PatinaFloat,
}

// We use this to nudge the origin of the bouncing ray slightly off the intersection point to avoid
// re-intersecting the same shape again.
const RAY_OFFSET_SCALE: PatinaFloat = 1e-9;

impl SurfaceInteraction {
    /// A ray leaving this point in direction `d`.
    ///
    /// The origin is moved off the surface along the normal, towards the side `d` leaves on, so the
    /// ray doesn't hit the surface it starts on.
    pub fn spawn_ray(&self, d: PatinaVec3f<RenderSpace>) -> PatinaRay3f<RenderSpace> {
        PatinaRay3f::new(self.offset_origin(d), d, self.time)
    }

    // The hit point, moved off the surface to the side of the normal that `d` points to.
    fn offset_origin(&self, d: PatinaVec3f<RenderSpace>) -> PatinaPoint3f<RenderSpace> {
        let magnitude = self.p.abs().max_component().max(1.0);
        let n = PatinaVec3f::new(self.n.x, self.n.y, self.n.z);
        let offset = n * (RAY_OFFSET_SCALE * magnitude);

        // Nudge the point slightly by the offset
        if d.dot(&n) < 0.0 {
            self.p + -offset
        } else {
            self.p + offset
        }
    }
}

// The result of a ray intersection with a quadric shape (sphere, disk), in object space. pbrt calls
// this QuadricIntersection. The shape turns it into a SurfaceInteraction in render space.
pub(crate) struct QuadraticIntersection {
    pub(crate) t_hit: PatinaFloat,
    pub(crate) p_object: PatinaPoint3f<ObjectSpace>,
    pub(crate) phi: PatinaFloat,
}

pub struct ShapeIntersection {
    pub interaction: SurfaceInteraction,
    /// The distance along the ray to the hit.
    pub t_hit: PatinaFloat,
}

pub struct Interaction;
pub struct ShapeSample {
    interaction: Interaction,
    pdf: PatinaFloat,
}

pub struct ShapeSampleContext {
    p: PatinaPoint3f<RenderSpace>,
    surface_normal: PatinaNorm3f<RenderSpace>,
    shading_normal: PatinaNorm3f<RenderSpace>,
    time: PatinaFloat,
}

pub trait Shape {
    /// Return the axis aligned bounding box encompassing this shape.
    fn bounds(&self) -> Bounds3f<RenderSpace>;
    /// Return a single ray-shape intersection (if any) corresponding to the first intersection
    /// in the interval [0, t_max]. With t_max = PatinaFloat::infinity if not set.
    ///
    /// Note that the ray is in rendering space. Must be converted by the struct implementing this
    /// trait to another space if needed.
    fn intersect(
        &self,
        ray: &PatinaRay3f<RenderSpace>,
        t_max: Option<PatinaFloat>,
    ) -> Option<ShapeIntersection>;

    /// Returns a boolean representing whether the ray intersects the shape or not in the interval
    /// [0, t_max].
    fn intersects(&self, ray: &PatinaRay3f<RenderSpace>, t_max: Option<PatinaFloat>) -> bool;

    // TODO: Implement direction-cone
    // fn normal_bounds() -> DirectionCone;

    /// Returns the surface area of the shape in question. Useful specifically for emitters.
    fn area(&self) -> PatinaFloat;

    /// Returns a point on the surface using a sampling distribution wrt to the surface area. The
    /// uniform u has to be in [0,
    /// 1)^2.
    fn sample(&self, u: PatinaPoint2f) -> Option<ShapeSample>;

    /// Returns the probability density for sampling the specified point, corresponding to the
    /// provided Interaction.
    fn pdf(&self, interaction: &Interaction) -> PatinaFloat;

    /// Returns a point on the surface using a distribution with respect to the solid angle from the
    /// reference point (provided in the sampling context).
    fn sample_from_point(&self, ctx: &ShapeSampleContext, u: PatinaPoint2f) -> Option<ShapeSample>;

    /// Returns the shape's probability of sampling a point on the light such that the incident
    /// direction at the point ctx.p is w_incident.
    fn pdf_from_point(
        &self,
        ctx: &ShapeSampleContext,
        w_incident: PatinaVec3f<RenderSpace>,
    ) -> PatinaFloat;
}

#[cfg(test)]
mod test_spawn_ray {
    use std::f64::consts::PI;

    use crate::{
        math::{PatinaFloat, PatinaPoint3f, PatinaRay3f, PatinaVec3f, RenderSpace, Transform},
        shapes::{Shape, Sphere},
    };

    // For a sphere at the given position and size, shoots rays at points spread over its front,
    // spawns a ray from each hit in a direction just above the tangent plane, and counts how many of
    // those re-hit the sphere. Leaving the surface that close to the tangent plane is the hardest
    // case for self-intersection.
    fn count_self_hits(center: PatinaFloat, radius: PatinaFloat) -> (usize, usize) {
        let sphere = Sphere::new(
            radius,
            Transform::translate(&PatinaVec3f::new(center, 0.0, center)),
            -radius,
            radius,
            2.0 * PI,
            false,
        );
        let n = 100;
        let (mut hits, mut self_hits) = (0, 0);
        for i in 0..n {
            for j in 0..n {
                let a = (i as PatinaFloat + 0.5) / n as PatinaFloat - 0.5;
                let b = (j as PatinaFloat + 0.5) / n as PatinaFloat - 0.5;
                let origin: PatinaPoint3f<RenderSpace> = PatinaPoint3f::new(
                    center + a * 1.6 * radius,
                    b * 1.6 * radius,
                    center - 10.0 * radius,
                );
                let ray = PatinaRay3f::new(origin, PatinaVec3f::new(0.0, 0.0, 1.0), 0.0);
                let Some(hit) = sphere.intersect(&ray, None) else {
                    continue;
                };
                hits += 1;

                let it = hit.interaction;
                let normal = PatinaVec3f::new(it.n.x, it.n.y, it.n.z);
                let tangent = PatinaVec3f::new(-normal.y, normal.x, 0.0).normalize();
                let d = (tangent + normal * 1e-3).normalize();
                if sphere.intersect(&it.spawn_ray(d), None).is_some() {
                    self_hits += 1;
                }
            }
        }
        (hits, self_hits)
    }

    #[test]
    fn test_spawned_rays_do_not_hit_their_own_surface() {
        // Without the offset, a large share of these rays re-hit the sphere, more so far from the
        // origin, where rounding errors are larger.
        for (center, radius) in [(0.0, 1.0), (100.0, 1.0), (1.0e4, 50.0)] {
            let (hits, self_hits) = count_self_hits(center, radius);
            assert!(hits > 0);
            assert_eq!(self_hits, 0, "sphere at {center} with radius {radius}");
        }
    }

    #[test]
    fn test_spawned_ray_leaves_on_the_side_of_its_direction() {
        // A hit on the near side of a sphere, whose normal points back along -z.
        let sphere = Sphere::new(
            1.0,
            Transform::translate(&PatinaVec3f::new(0.0, 0.0, 5.0)),
            -1.0,
            1.0,
            2.0 * PI,
            false,
        );
        let ray = PatinaRay3f::new(
            PatinaPoint3f::new(0.2, 0.1, 0.0),
            PatinaVec3f::new(0.0, 0.0, 1.0),
            0.75,
        );
        let it = sphere
            .intersect(&ray, None)
            .expect("ray should hit")
            .interaction;
        let n = PatinaVec3f::new(it.n.x, it.n.y, it.n.z);

        // Out of the surface: the origin moves outwards, and the time is kept.
        let out = it.spawn_ray(n);
        assert!((out.o - it.p).dot(&n) > 0.0);
        assert_eq!(out.t, 0.75);

        // Into the surface: the origin moves inwards, so the ray hits the far side of the sphere
        // rather than the point it started at.
        let inward = it.spawn_ray(-n);
        assert!((inward.o - it.p).dot(&n) < 0.0);
        let far = sphere.intersect(&inward, None).expect("the far side");
        assert!(far.t_hit > 1.0);
    }
}
