pub(crate) mod sphere;

use crate::math::{
    Bounds3f, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, PatinaVec3f,
};

pub struct SurfaceInteraction;
pub struct ShapeIntersection {
    interaction: SurfaceInteraction,
    t_hit: PatinaFloat,
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
    fn bounds() -> Bounds3f;
    /// Return a single ray-shape intersection (if any) corresponding to the first intersection
    /// in the interval [0, t_max]. With t_max = PatinaFloat::infinity if not set.
    ///
    /// Note that the ray is in rendering space. Must be converted by the struct implementing this
    /// trait to another space if needed.
    fn intersect(ray: &PatinaRay3f, t_max: Option<PatinaFloat>) -> Option<ShapeIntersection>;

    /// Returns a boolean representing whether the ray intersects the shape or not in the interval
    /// [0, t_max].
    fn intersects(ray: &PatinaRay3f, t_max: Option<PatinaFloat>) -> bool;

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
    fn sample_from_point(ctx: &ShapeSampleContext, u: PatinaPoint2f) -> Option<ShapeSample>;

    /// Returns the shape's probability of sampling a point on the light such that the incident
    /// direction at the point ctx.p is w_incident.
    fn pdf_from_point(ctx: &ShapeSampleContext, w_incident: PatinaVec3f) -> PatinaFloat;
}
