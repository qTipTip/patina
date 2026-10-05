use std::ops::Index;

use num_traits::{Bounded, Num};

use crate::math::{points::point3::Point3, traits::CheckNan};

pub struct Bounds3<T> {
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
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
