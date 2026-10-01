use std::ops::Add;

use crate::math::traits::CheckNan;
use crate::math::tuples::tuple2::Tuple2;
use crate::math::tuples::{TupleLength, TypePoint};
use crate::math::vectors::vec2::Vector2;
use crate::math::{PatinaFloat, PatinaInt};
use num_traits::{Num, ToPrimitive};

// Point - Point = Vector comes from the `Difference` impl on `TypePoint`.
pub type Point2<T> = Tuple2<T, TypePoint>;

pub type PatinaPoint2f = Point2<PatinaFloat>;
pub type PatinaPoint2i = Point2<PatinaInt>;

impl<T> Point2<T>
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
impl<T: Num + Copy + CheckNan> Add<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(test)]
mod point_vector_interactions {
    use crate::math::{points::point2::Point2, vectors::vec2::Vector2};

    #[test]
    fn point_add_vector_is_point() {
        let p = Point2::new(0.0, 1.0);
        let q = Vector2::new(1.0, 0.0);

        assert_eq!(p + q, Point2::new(1.0, 1.0));
    }

    #[test]
    fn point_sub_point_is_vector() {
        let p = Point2::new(0.0, 1.0);
        let q = Point2::new(1.0, 0.0);
        assert_eq!(p - q, Vector2::new(-1.0, 1.0));
    }

    #[test]
    fn test_point_distance() {
        let p = Point2::new(0.0, 1.0);
        let q = Point2::new(1.0, 0.0);

        assert_eq!(p.distance(q), 2.0_f64.sqrt());
        assert_eq!(p.distance_squared(q), 2.0);
    }
}
