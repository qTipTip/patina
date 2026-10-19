use crate::{
    math::{Bounds3f, PatinaFloat, PatinaPoint2f, PatinaRay3f, PatinaVec3f, Transform},
    shapes::Shape,
};

use super::{ShapeSample, ShapeSampleContext};

/// A sphere is sweeped out using polar coordinates around the origin (in object space). A
/// transformation (render_from_object/object_from_render) is used to place the sphere in render
/// space.
pub struct Sphere {
    // Radius of the sphere
    radius: PatinaFloat,
    // Whether to reverse the direction of surface normals to point inwards.
    reverse_orientation: bool,
    // Places the object in rendering space.
    render_from_object: Transform,
    object_from_render: Transform,
    // z_min and z_max for truncated sphere if desired
    z_min: PatinaFloat,
    z_max: PatinaFloat,
    // phi_max for partial sphere
    phi_max: PatinaFloat,
}

impl Sphere {
    pub fn new(
        radius: PatinaFloat,
        render_from_object: Transform,
        z_min: PatinaFloat,
        z_max: PatinaFloat,
        phi_max: PatinaFloat,
        reverse_orientation: bool,
    ) -> Self {
        let object_from_render = render_from_object.inverse();
        Self {
            radius,
            reverse_orientation,
            render_from_object,
            object_from_render,
            z_min,
            z_max,
            phi_max,
        }
    }
}

impl Shape for Sphere {
    fn bounds() -> Bounds3f {
        todo!()
    }

    fn intersect(
        ray: &PatinaRay3f,
        t_max: Option<PatinaFloat>,
    ) -> Option<super::ShapeIntersection> {
        todo!()
    }

    fn intersects(ray: &PatinaRay3f, t_max: Option<PatinaFloat>) -> bool {
        todo!()
    }

    fn area(&self) -> PatinaFloat {
        todo!()
    }

    fn sample(&self, u: PatinaPoint2f) -> Option<ShapeSample> {
        todo!()
    }

    fn pdf(&self, interaction: &super::Interaction) -> PatinaFloat {
        todo!()
    }

    fn sample_from_point(ctx: &ShapeSampleContext, u: PatinaPoint2f) -> Option<ShapeSample> {
        todo!()
    }

    fn pdf_from_point(ctx: &ShapeSampleContext, w_incident: PatinaVec3f) -> PatinaFloat {
        todo!()
    }
}
