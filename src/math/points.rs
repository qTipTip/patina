use std::ops::{Add, Deref, DerefMut};

use crate::math::traits::CheckNan;
use crate::math::tuples::tuple2::Tuple2;
use crate::math::tuples::tuple3::Tuple3;
use crate::math::vectors::vec3::Vector3;
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

impl<T: Num + Copy + CheckNan> Point2<T> {
    pub fn new(x: T, y: T) {
        Self(Tuple2::new(x, y));
    }
}
impl<T: Num + Copy + CheckNan> Point3<T> {
    pub fn new(x: T, y: T, z: T) {
        Self(Tuple3::new(x, y, z));
    }
}

// Point / Vector operation

impl<T> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {}
}
