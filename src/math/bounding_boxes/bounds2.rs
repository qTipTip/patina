use std::ops::Index;

use num_traits::{Bounded, Num};

use crate::math::{points::point2::Point2, traits::CheckNan};

#[derive(PartialEq, Debug)]
pub struct Bounds2<T> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl<T: Num + Bounded + Copy + CheckNan> Default for Bounds2<T> {
    fn default() -> Self {
        // Create a default "empty" (but invalid) bounding box, where pMin is set to the largest
        // possible values, and pMax is set to the lowest.
        let min = T::min_value();
        let max = T::max_value();
        Self {
            p_min: Point2::new(max, max),
            p_max: Point2::new(min, min),
        }
    }
}

// Instantiate a bounding box from a single point.
impl<T: Copy> From<Point2<T>> for Bounds2<T> {
    fn from(value: Point2<T>) -> Self {
        Self {
            p_min: value,
            p_max: value,
        }
    }
}

impl<T: Num + Copy + PartialOrd + CheckNan> Bounds2<T> {
    pub fn new(p1: Point2<T>, p2: Point2<T>) -> Self {
        Self {
            p_min: Point2::min(p1, p2),
            p_max: Point2::max(p1, p2),
        }
    }

    pub fn from_point(p1: Point2<T>) -> Self {
        Self::from(p1)
    }

    // Return the i'th out of the eight corners of the bounding box.
    pub fn get_corner(&self, corner: usize) -> Point2<T> {
        // There are exactly four possible combination of the corner & 1, corner & 2 and corner & 4
        // indices, so they enumerate the corners.
        Point2::new(
            self[corner & 1].x,
            self[if (corner & 2) != 0 { 1 } else { 0 }].y,
        )
    }

    // Returns the union of between this and another bounding box.
    pub fn union(&self, rhs: &Self) -> Self {
        Self {
            p_min: Point2::min(self.p_min, rhs.p_min),
            p_max: Point2::max(self.p_max, rhs.p_max),
        }
    }

    // Returns the intersection between this and another bounding box
    pub fn intersection(&self, rhs: &Self) -> Self {
        Self {
            p_min: Point2::max(self.p_min, rhs.p_min),
            p_max: Point2::min(self.p_max, rhs.p_max),
        }
    }

    // Returns a bounding box that encompasses the point.
    pub fn union_point(&self, p: Point2<T>) -> Self {
        Self {
            p_min: Point2::min(self.p_min, p),
            p_max: Point2::max(self.p_max, p),
        }
    }
}

impl<T> Index<usize> for Bounds2<T> {
    type Output = Point2<T>;

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
    use crate::math::{bounding_boxes::bounds2::Bounds2, points::PatinaPoint2f};

    #[test]
    fn test_corner_enumeration() {
        let b = Bounds2::new(PatinaPoint2f::new(0.0, 0.0), PatinaPoint2f::new(1.0, 1.0));

        let expected_corners = [
            PatinaPoint2f::new(0.0, 0.0),
            PatinaPoint2f::new(1.0, 0.0),
            PatinaPoint2f::new(0.0, 1.0),
            PatinaPoint2f::new(1.0, 1.0),
            PatinaPoint2f::new(0.0, 0.0),
            PatinaPoint2f::new(1.0, 0.0),
            PatinaPoint2f::new(0.0, 1.0),
            PatinaPoint2f::new(1.0, 1.0),
        ];

        for (i, &corner) in expected_corners.iter().enumerate() {
            assert_eq!(corner, b.get_corner(i), "corner {i} didn't match expected")
        }
    }

    #[test]
    fn test_bounds_union() {
        let a = Bounds2::new(PatinaPoint2f::new(0.0, 0.0), PatinaPoint2f::new(2.0, 2.0));
        let b = Bounds2::new(PatinaPoint2f::new(1.0, 1.0), PatinaPoint2f::new(3.0, 3.0));

        assert_eq!(
            a.union(&b),
            Bounds2::new(PatinaPoint2f::new(0.0, 0.0), PatinaPoint2f::new(3.0, 3.0))
        );
    }

    #[test]
    fn test_bounds_point_union() {
        let a = Bounds2::new(PatinaPoint2f::new(0.0, 0.0), PatinaPoint2f::new(2.0, 2.0));
        let p = PatinaPoint2f::new(1.0, 1.0);

        assert_eq!(a.union_point(p), a);
    }

    #[test]
    fn test_bounds_intersection() {
        let a = Bounds2::new(PatinaPoint2f::new(0.0, 0.0), PatinaPoint2f::new(2.0, 2.0));
        let b = Bounds2::new(PatinaPoint2f::new(1.0, 1.0), PatinaPoint2f::new(3.0, 3.0));

        assert_eq!(
            a.intersection(&b),
            Bounds2::new(PatinaPoint2f::new(1.0, 1.0), PatinaPoint2f::new(2.0, 2.0))
        );
    }
}
