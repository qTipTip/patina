use crate::math::traits::CheckNan;
use crate::{
    implement_geometry_ops,
    math::{PatinaFloat, PatinaInt},
    math::{vectors::vec2::Vector2, vectors::vec3::Vector3},
};
use num_traits::Num;

#[derive(Copy)]
pub struct Point2<T>(Vector2<T>);
#[derive(Copy)]
pub struct Point3<T>(Vector3<T>);

implement_geometry_ops!(Point2, Vector2);
implement_geometry_ops!(Point3, Vector3);

type PatinaPoint3f = Point3<PatinaFloat>;

type PatinaPoint3i = Point3<PatinaInt>;
type PatinaPoint2f = Point2<PatinaFloat>;
type PatinaPoint2i = Point2<PatinaInt>;
