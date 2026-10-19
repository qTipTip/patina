#![allow(dead_code)]
pub(crate) mod sphere;

pub use sphere::Sphere;

use crate::math::{
    Bounds3f, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, PatinaVec3f,
};

/// The local geometry at a ray-surface hit, in render space. A minimal version of pbrt's
/// SurfaceInteraction: partial derivatives (dpdu, dpdv, dndu, dndv) are left out until a later
/// chapter needs them.
pub struct SurfaceInteraction {
    /// The hit point.
    pub p: PatinaPoint3f,
    /// The geometric surface normal, normalized. Points outwards unless the shape's orientation
    /// is reversed.
    pub n: PatinaNorm3f,
    /// The shape's (u, v) parameterization of the hit point, each in [0, 1].
    pub uv: PatinaPoint2f,
    /// The normalized direction back along the ray, towards its origin.
    pub wo: PatinaVec3f,
    /// The time of the ray that produced the hit.
    pub time: PatinaFloat,
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
    p: PatinaPoint3f,
    surface_normal: PatinaNorm3f,
    shading_normal: PatinaNorm3f,
    time: PatinaFloat,
}

pub trait Shape {
    /// Return the axis aligned bounding box encompassing this shape.
    fn bounds(&self) -> Bounds3f;
    /// Return a single ray-shape intersection (if any) corresponding to the first intersection
    /// in the interval [0, t_max]. With t_max = PatinaFloat::infinity if not set.
    ///
    /// Note that the ray is in rendering space. Must be converted by the struct implementing this
    /// trait to another space if needed.
    fn intersect(&self, ray: &PatinaRay3f, t_max: Option<PatinaFloat>)
    -> Option<ShapeIntersection>;

    /// Returns a boolean representing whether the ray intersects the shape or not in the interval
    /// [0, t_max].
    fn intersects(&self, ray: &PatinaRay3f, t_max: Option<PatinaFloat>) -> bool;

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
    fn pdf_from_point(&self, ctx: &ShapeSampleContext, w_incident: PatinaVec3f) -> PatinaFloat;
}
