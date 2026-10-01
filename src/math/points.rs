use std::ops::{Add, Deref, DerefMut, Sub};

use crate::math::traits::{CheckNan, IsNotFloat};
use crate::math::tuples::tuple2::Tuple2;
use crate::math::tuples::tuple3::Tuple3;
use crate::math::vectors::vec3::Vector3;
use crate::{
    implement_geometry_ops,
    math::{PatinaFloat, PatinaInt},
};
use num_traits::{Num, ToPrimitive};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point2<T>(Tuple2<T>);
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point3<T>(Tuple3<T>);

// We don't inherit the Sub operator, as we'd like Point - Point = Vec. and not Point.
implement_geometry_ops!(Point2, Tuple2, [Add, Mul, Neg, Div]);
implement_geometry_ops!(Point3, Tuple3, [Add, Mul, Neg, Div]);

pub type PatinaPoint3f = Point3<PatinaFloat>;
pub type PatinaPoint3i = Point3<PatinaInt>;
pub type PatinaPoint2f = Point2<PatinaFloat>;
pub type PatinaPoint2i = Point2<PatinaInt>;

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
    pub fn new(x: T, y: T) -> Self {
        Self(Tuple2::new(x, y))
    }
}
impl<T: Num + Copy + CheckNan> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple3::new(x, y, z))
    }
}

// Point / Vector operation
impl<T: Num + Copy + CheckNan + std::ops::Add> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Num + Sub + CheckNan + ToPrimitive + Copy> Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod point_vector_interactions {
    use crate::math::{points::Point3, vectors::vec3::Vector3};

    #[test]
    fn point_add_vector_is_point() {
        let p = Point3::new(0.0, 1.0, 2.0);
        let q = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(p + q, Point3::new(1.0, 1.0, 2.0));
    }

    #[test]
    fn point_sub_point_is_vector() {
        let p = Point3::new(0.0, 1.0, 2.0);
        let q = Point3::new(1.0, 0.0, 0.0);
        assert_eq!(p - q, Vector3::new(-1.0, 1.0, 2.0));
    }
}
