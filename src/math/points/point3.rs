use std::ops::Add;

use crate::math::traits::CheckNan;
use crate::math::tuples::tuple3::Tuple3;
use crate::math::tuples::{TupleLength, TypePoint};
use crate::math::vectors::vec3::Vector3;
use num_traits::{Num, ToPrimitive};

// Point - Point = Vector comes from the `Difference` impl on `TypePoint`.
pub type Point3<T> = Tuple3<T, TypePoint>;

impl<T> Point3<T>
where
    T: Num + CheckNan + ToPrimitive + Copy,
{
    pub fn distance(self, rhs: Self) -> TupleLength {
        (self - rhs).len()
    }
    pub fn distance_squared(self, rhs: Self) -> T {
        (self - rhs).len_squared()
    }
}

// Point / Vector operation
impl<T: Num + Copy + CheckNan> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod point_vector_interactions {

    use crate::math::{points::point3::Point3, vectors::vec3::Vector3};

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

    #[test]
    fn test_point_distance() {
        let p = Point3::new(0.0, 1.0, 2.0);
        let q = Point3::new(1.0, 0.0, 0.0);

        assert_eq!(p.distance(q), 6.0_f64.sqrt());
        assert_eq!(p.distance_squared(q), 6.0);
    }
}
