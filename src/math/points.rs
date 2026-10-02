use crate::math::{
    PatinaFloat, PatinaInt,
    points::{point2::Point2, point3::Point3},
};

pub mod point2;
pub mod point3;

pub type PatinaPoint3f = Point3<PatinaFloat>;
pub type PatinaPoint3i = Point3<PatinaInt>;

pub type PatinaPoint2f = Point2<PatinaFloat>;
pub type PatinaPoint2i = Point2<PatinaInt>;
