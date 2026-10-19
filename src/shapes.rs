pub(crate) mod sphere;

use crate::math::{Bounds3f, PatinaFloat, PatinaRay3f};

pub struct SurfaceInteraction;
pub struct ShapeIntersection {
    interaction: SurfaceInteraction,
    t_hit: PatinaFloat,
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
}
