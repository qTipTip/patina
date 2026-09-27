use std::ops::{self, AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

use num_traits::Float;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub(crate) struct Tuple3<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Tuple3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let t = Self { x, y, z };
        debug_assert!(!t.has_nan());
        t
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn has_nan(&self) -> bool {
        T::is_nan(self.x) || T::is_nan(self.y) || T::is_nan(self.z)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl<T: Float> Index<usize> for Tuple3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &self.x;
        };
        if index == 1 {
            return &self.y;
        };
        if index == 2 {
            return &self.z;
        }
        panic!("Indexing into item with length 3, with index {index}")
    }
}

impl<T: Float> IndexMut<usize> for Tuple3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            return &mut self.x;
        }
        if index == 1 {
            return &mut self.y;
        }
        if index == 2 {
            return &mut self.z;
        }
        panic!("Indexing into item with length 3, with index {index}")
    }
}

impl<T: Float> ops::Add for Tuple3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Float + AddAssign> ops::AddAssign for Tuple3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Float> ops::Neg for Tuple3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Float> ops::Sub for Tuple3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Float + SubAssign> ops::SubAssign for Tuple3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Float> ops::Mul<T> for Tuple3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<T: Float + MulAssign> ops::MulAssign<T> for Tuple3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Float> ops::Div<T> for Tuple3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Float + DivAssign> ops::DivAssign<T> for Tuple3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;

        debug_assert!(!self.has_nan())
    }
}

#[cfg(test)]
mod test_construction {
    use approx::assert_relative_eq;

    use crate::math::{PatinaFloat, tuples::tuple3::Tuple3};

    #[test]
    fn test_tuples_3() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 1.0);
        assert_relative_eq!(t[2], 2.0);
    }

    #[test]
    fn test_tuples_3_zero() {
        let t = Tuple3::<PatinaFloat>::zero();
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 0.0);
        assert_relative_eq!(t[2], 0.0);
    }
}

#[cfg(test)]
mod test_operations {
    use crate::math::{PatinaFloat, tuples::tuple3::Tuple3};

    #[test]
    fn test_addition() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat>::new(1.5, 3.0, 2.0);

        assert_eq!(t + r, Tuple3::<PatinaFloat>::new(1.5, 4.0, 4.0));
    }

    #[test]
    fn test_additive_identity() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        let o = Tuple3::<PatinaFloat>::zero();

        assert_eq!(t + o, t);
    }

    #[test]
    fn test_add_assign() {
        let mut t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat>::new(1.5, 3.0, 2.0);
        t += r;

        assert_eq!(t, Tuple3::<PatinaFloat>::new(1.5, 4.0, 4.0));
    }

    #[test]
    fn test_negation() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);

        assert_eq!(-t, Tuple3::<PatinaFloat>::new(0.0, -1.0, -2.0));
    }

    #[test]
    fn test_subtraction() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat>::new(1.5, 3.0, 2.0);

        assert_eq!(t - r, Tuple3::<PatinaFloat>::new(-1.5, -2.0, 0.0));
    }

    #[test]
    fn test_subassign() {
        let mut t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        t -= t;

        assert_eq!(t, Tuple3::<PatinaFloat>::zero());
    }

    #[test]
    fn test_scalar_mult() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        assert_eq!(t * 2.0, Tuple3::<PatinaFloat>::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn test_scalar_mult_assign() {
        let mut t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        t *= 2.0;
        assert_eq!(t, Tuple3::<PatinaFloat>::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn test_division() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);

        assert_eq!(t / 2.0, Tuple3::<PatinaFloat>::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn test_div_assign() {
        let mut t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        t /= 2.0;
        assert_eq!(t, Tuple3::<PatinaFloat>::new(0.0, 0.5, 1.0));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero_panics() {
        let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        let _ = t / 0.0;
    }

    #[test]
    #[should_panic]
    fn test_div_assign_by_zero_panics() {
        let mut t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
        t /= 0.0;
    }
}

#[cfg(test)]
mod test_tuple3_functions {
    use crate::math::{PatinaFloat, tuples::tuple3::Tuple3};

    #[test]
    fn test_absolute_value() {
        let t = Tuple3::<PatinaFloat>::new(-1.3, 1.0, -12.0);
        assert_eq!(t.abs(), Tuple3::<PatinaFloat>::new(1.3, 1.0, 12.0));
    }

    #[test]
    fn test_ceil_and_floor() {
        let t = Tuple3::<PatinaFloat>::new(-1.3, 1.1, 2.8);
        assert_eq!(t.ceil(), Tuple3::<PatinaFloat>::new(-1.0, 2.0, 3.0));
        assert_eq!(t.floor(), Tuple3::<PatinaFloat>::new(-2.0, 1.0, 2.0));
    }
}
