use std::ops::{Deref, DerefMut};

use crate::math::traits::CheckNan;
use crate::math::tuples::tuple2::Tuple2;
use crate::math::tuples::tuple3::Tuple3;
use crate::{
    implement_geometry_ops,
    math::{PatinaFloat, PatinaInt},
};
use num_traits::Num;

#[derive(Copy, Clone)]
pub struct Point2<T>(Tuple2<T>);
#[derive(Copy, Clone)]
pub struct Point3<T>(Tuple3<T>);

implement_geometry_ops!(Point2, Tuple2);
implement_geometry_ops!(Point3, Tuple3);

type PatinaPoint3f = Point3<PatinaFloat>;
type PatinaPoint3i = Point3<PatinaInt>;
type PatinaPoint2f = Point2<PatinaFloat>;
type PatinaPoint2i = Point2<PatinaInt>;

impl<T> DerefMut for Point2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for Point2<T> {
    type Target = Tuple2<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Point3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for Point3<T> {
    type Target = Tuple3<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl From<PatinaPoint2f> for PatinaPoint2i {
//     fn from(value: PatinaPoint2f) -> Self {
//         Self
//     }
// }
// impl From<PatinaPoint2i> for PatinaPoint2f {
//     fn from(value: PatinaPoint2i) -> Self {
//         todo!()
//     }
// }
// impl From<PatinaPoint3f> for PatinaPoint3i {
//     fn from(value: PatinaPoint3f) -> Self {
//         todo!()
//     }
// }
// impl From<PatinaPoint3i> for PatinaPoint3f {
//     fn from(value: PatinaPoint3i) -> Self {
//         todo!()
//     }
// }
