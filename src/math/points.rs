use crate::math::{
    PatinaFloat, PatinaInt,
    points::{point2::Point2, point3::Point3},
};

pub mod point2;
pub mod point3;

pub type PatinaPoint3f<S> = Point3<PatinaFloat, S>;
pub type PatinaPoint3i<S> = Point3<PatinaInt, S>;

pub type PatinaPoint2f = Point2<PatinaFloat>;
pub type PatinaPoint2i = Point2<PatinaInt>;
pub type PatinaPoint2u = Point2<u32>;
