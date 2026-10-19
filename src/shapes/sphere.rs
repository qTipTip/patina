use crate::{
    math::{Bounds3f, PatinaFloat, PatinaPoint2f, PatinaRay3f, PatinaVec3f},
    shapes::Shape,
};

use super::{ShapeSample, ShapeSampleContext};

pub struct Sphere;

impl Sphere {}

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
