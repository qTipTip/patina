use std::{mem::swap, ops::Index};

use approx::{AbsDiffEq, RelativeEq};
use num_traits::{Bounded, Float, Num};

use crate::math::{
    PatinaFloat, PatinaPoint3f, PatinaVec3f, points::point3::Point3, traits::CheckNan,
};

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

pub struct BoundsIntersection {
    t0: PatinaFloat,
    t1: PatinaFloat,
}
impl<T: Float + Copy + PartialOrd + CheckNan> Bounds3<T> {
    pub fn intersects(
        &self,
        o: &PatinaPoint3f,
        d: &PatinaVec3f,
        t_max: Option<PatinaFloat>,
    ) -> Option<BoundsIntersection> {
        let mut t0 = 0.0;
        let mut t1 = t_max.unwrap_or(f64::INFINITY);
        for i in 0..3 {
            let ray_dir_inv = 1.0 / d[i];
            let mut t_near = (self.p_min[i].to_f64().unwrap() - o[i]) * ray_dir_inv;
            let mut t_far = (self.p_max[i].to_f64().unwrap() - o[i]) * ray_dir_inv;

            if t_near > t_far {
                swap(&mut t_near, &mut t_far);
            };

            if t_near > t0 {
                t0 = t_near;
            };
            if t_far < t1 {
                t1 = t_far;
            };
            if t0 > t1 {
                return None;
            };
        }
        Some(BoundsIntersection { t0, t1 })
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

#[cfg(test)]
mod test_ray_interactions {
    use approx::assert_relative_eq;

    use crate::math::{Bounds3, PatinaPoint3f, PatinaRay3f, PatinaVec3f};

    #[test]
    fn test_ray_bounds_intersection() {
        let a = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(2.0, 2.0, 2.0),
        );

        let r = PatinaRay3f::new(
            PatinaPoint3f::new(-1.0, -1.0, -1.0),
            PatinaVec3f::new(1.0, 1.0, 1.0),
            0.0,
        );

        let res = a.intersects(&r.o, &r.d, None);
        assert!(res.is_some());
        let i = res.unwrap();

        assert!(i.t0 < i.t1);
        assert_relative_eq!(i.t0, 1.0);
        assert_relative_eq!(i.t1, 3.0);

        assert_relative_eq!(r.eval(i.t0), PatinaPoint3f::zero());
        assert_relative_eq!(r.eval(i.t1), PatinaPoint3f::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_ray_bounds_no_intersection() {
        let a = Bounds3::new(
            PatinaPoint3f::new(0.0, 0.0, 0.0),
            PatinaPoint3f::new(2.0, 2.0, 2.0),
        );

        let r = PatinaRay3f::new(
            PatinaPoint3f::new(3.0, 3.0, 3.0),
            PatinaVec3f::new(1.0, 1.0, 1.0),
            0.0,
        );

        let res = a.intersects(&r.o, &r.d, None);
        assert!(res.is_none());
    }
}
