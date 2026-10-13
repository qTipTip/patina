use std::ops::Index;

use approx::{AbsDiffEq, RelativeEq};
use num_traits::{Bounded, Num};

use crate::math::{PatinaFloat, points::point3::Point3, traits::CheckNan};

#[derive(PartialEq, Debug)]
pub struct Bounds3<T> {
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

impl AbsDiffEq for Bounds3<PatinaFloat> {
    type Epsilon = PatinaFloat;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.p_min.abs_diff_eq(&other.p_min, epsilon)
            && self.p_max.abs_diff_eq(&other.p_max, epsilon)
    }
}

impl RelativeEq for Bounds3<PatinaFloat> {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-12
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.p_min.relative_eq(&other.p_min, epsilon, max_relative)
            && self.p_max.relative_eq(&other.p_max, epsilon, max_relative)
    }
}

impl<T: Num + Bounded + Copy + CheckNan> Default for Bounds3<T> {
    fn default() -> Self {
        // Create a default "empty" (but invalid) bounding box, where pMin is set to the largest
        // possible values, and pMax is set to the lowest.
        let min = T::min_value();
        let max = T::max_value();
        Self {
            p_min: Point3::new(max, max, max),
            p_max: Point3::new(min, min, min),
        }
    }
}

// Instantiate a bounding box from a single point.
impl<T: Copy> From<Point3<T>> for Bounds3<T> {
    fn from(value: Point3<T>) -> Self {
        Self {
            p_min: value,
            p_max: value,
        }
    }
}

impl<T: Num + Copy + PartialOrd + CheckNan> Bounds3<T> {
    pub fn new(p1: Point3<T>, p2: Point3<T>) -> Self {
        Self {
            p_min: Point3::min(p1, p2),
            p_max: Point3::max(p1, p2),
        }
    }

    pub fn from_point(p1: Point3<T>) -> Self {
        Self::from(p1)
    }

    // Return the i'th out of the eight corners of the bounding box.
    pub fn get_corner(&self, corner: usize) -> Point3<T> {
        // There are exactly eight possible combination of the corner & 1, corner & 2 and corner & 4
        // indices, so they enumerate the corners.
        Point3::new(
            self[corner & 1].x,
            self[if (corner & 2) != 0 { 1 } else { 0 }].y,
            self[if (corner & 4) != 0 { 1 } else { 0 }].z,
        )
    }

    // Returns the union of between this and another bounding box.
    pub fn union(&self, rhs: &Self) -> Self {
        Self {
            p_min: Point3::min(self.p_min, rhs.p_min),
            p_max: Point3::max(self.p_max, rhs.p_max),
        }
    }

    // Returns the intersection between this and another bounding box
    pub fn intersection(&self, rhs: &Self) -> Self {
        Self {
            p_min: Point3::max(self.p_min, rhs.p_min),
            p_max: Point3::min(self.p_max, rhs.p_max),
        }
    }

    // Returns a bounding box that encompasses the point.
    pub fn union_point(&self, p: Point3<T>) -> Self {
        Self {
            p_min: Point3::min(self.p_min, p),
            p_max: Point3::max(self.p_max, p),
        }
    }
}

impl<T> Index<usize> for Bounds3<T> {
    type Output = Point3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(
            index == 0 || index == 1,
            "Can only index bounding boxes with 0 or 1"
        );

        if index == 0 { &self.p_min } else { &self.p_max }
    }
}

#[cfg(test)]
mod test {

    use crate::math::{bounding_boxes::bounds3::Bounds3, points::PatinaPoint3f};

    #[test]
    fn test_corner_enumeration() {
        let b = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(1.0, 1.0, 1.0),
        );

        let expected_corners = [
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(1.0, 0.0, 0.0),
            PatinaPoint3f::new(0.0, 1.0, 0.0),
            PatinaPoint3f::new(1.0, 1.0, 0.0),
            PatinaPoint3f::new(0.0, 0.0, 1.0),
            PatinaPoint3f::new(1.0, 0.0, 1.0),
            PatinaPoint3f::new(0.0, 1.0, 1.0),
            PatinaPoint3f::new(1.0, 1.0, 1.0),
        ];

        for (i, &corner) in expected_corners.iter().enumerate() {
            assert_eq!(corner, b.get_corner(i), "corner {i} didn't match expected")
        }
    }

    #[test]
    fn test_bounds_union() {
        let a = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(2.0, 2.0, 2.0),
        );
        let b = Bounds3::new(
            PatinaPoint3f::new(1.0, 1.0, 1.0),
            PatinaPoint3f::new(3.0, 3.0, 3.0),
        );

        assert_eq!(
            a.union(&b),
            Bounds3::new(
                PatinaPoint3f::new(0.0, 0.0, 0.0),
                PatinaPoint3f::new(3.0, 3.0, 3.0)
            )
        );
    }

    #[test]
    fn test_bounds_point_union() {
        let a = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(2.0, 2.0, 2.0),
        );
        let p = PatinaPoint3f::new(1.0, 1.0, 1.0);

        assert_eq!(a.union_point(p), a);
    }

    #[test]
    fn test_bounds_intersection() {
        let a = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(2.0, 2.0, 2.0),
        );
        let b = Bounds3::new(
            PatinaPoint3f::new(1.0, 1.0, 1.0),
            PatinaPoint3f::new(3.0, 3.0, 3.0),
        );

        assert_eq!(
            a.intersection(&b),
            Bounds3::new(
                PatinaPoint3f::new(1.0, 1.0, 1.0),
                PatinaPoint3f::new(2.0, 2.0, 2.0)
            )
        );
    }
}
