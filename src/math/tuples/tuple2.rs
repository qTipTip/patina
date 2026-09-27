use std::ops::{self, AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg, SubAssign};

use num_traits::{Float, Num, float::FloatCore};
pub trait CheckNan {
    fn is_nan_val(&self) -> bool;
}

impl CheckNan for f32 {
    #[inline]
    fn is_nan_val(&self) -> bool {
        self.is_nan()
    }
}
impl CheckNan for f64 {
    #[inline]
    fn is_nan_val(&self) -> bool {
        self.is_nan()
    }
}

pub trait IsNotFloat {}
impl IsNotFloat for i8 {}
impl IsNotFloat for i16 {}
impl IsNotFloat for i32 {}
impl IsNotFloat for i64 {}
impl IsNotFloat for isize {}
impl IsNotFloat for u8 {}
impl IsNotFloat for u16 {}
impl IsNotFloat for u32 {}
impl IsNotFloat for u64 {}
impl IsNotFloat for usize {}

impl<T: IsNotFloat> CheckNan for T {
    #[inline]
    fn is_nan_val(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub(crate) struct Tuple2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num + CheckNan + Copy> Tuple2<T> {
    pub fn new(x: T, y: T) -> Self {
        debug_assert!(
            !x.is_nan_val() && !y.is_nan_val(),
            "Tuple2 components cannot be NaN!"
        );
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    pub fn lerp(t: T, a: Self, b: Self) -> Self {
        a * (T::one() - t) + b * t
    }

    pub fn hprod(&self) -> T {
        self.x * self.y
    }

    pub fn permute(&self, perm_indices: &[usize; 2]) -> Self {
        Self::new(self[perm_indices[0]], self[perm_indices[1]])
    }
}

impl<T: Num + CheckNan + Copy + PartialOrd> Tuple2<T> {
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(
            if a.x <= b.x { a.x } else { b.x },
            if a.y <= b.y { a.y } else { b.y },
        )
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self::new(
            if a.x >= b.x { a.x } else { b.x },
            if a.y >= b.y { a.y } else { b.y },
        )
    }

    pub fn min_component(&self) -> T {
        if self.x <= self.y { self.x } else { self.y }
    }

    pub fn max_component(&self) -> T {
        if self.x >= self.y { self.x } else { self.y }
    }

    pub fn min_component_index(&self) -> usize {
        if self.x <= self.y { 0 } else { 1 }
    }

    pub fn max_component_index(&self) -> usize {
        if self.x >= self.y { 0 } else { 1 }
    }
}

impl<T: num_traits::Signed + CheckNan + Copy> Tuple2<T> {
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl<T: Num + FloatCore + CheckNan + Copy> Tuple2<T> {
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Float + CheckNan> Tuple2<T> {
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn fma(a: Self, b: Self, c: Self) -> Self {
        a * b + c
    }
}

impl<T> Index<usize> for Tuple2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Indexing into item with length 2, with index {index}"),
        }
    }
}

impl<T> IndexMut<usize> for Tuple2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Indexing into item with length 2, with index {index}"),
        }
    }
}

impl<T: Num + CheckNan + Copy> ops::Add for Tuple2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> ops::AddAssign for Tuple2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy> ops::Neg for Tuple2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy> ops::Sub for Tuple2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: SubAssign> ops::SubAssign for Tuple2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Num + CheckNan + Copy> ops::Mul<T> for Tuple2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: MulAssign + Copy> ops::MulAssign<T> for Tuple2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Num + CheckNan + Copy> ops::Mul<Tuple2<T>> for Tuple2<T> {
    type Output = Self;
    fn mul(self, rhs: Tuple2<T>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: MulAssign> ops::MulAssign<Tuple2<T>> for Tuple2<T> {
    fn mul_assign(&mut self, rhs: Tuple2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Num + CheckNan + Copy> ops::Div<Tuple2<T>> for Tuple2<T> {
    type Output = Self;
    fn div(self, rhs: Tuple2<T>) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: DivAssign> ops::DivAssign<Tuple2<T>> for Tuple2<T> {
    fn div_assign(&mut self, rhs: Tuple2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Num + CheckNan + Copy> ops::Div<T> for Tuple2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: DivAssign + Copy + CheckNan> ops::DivAssign<T> for Tuple2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val(),
            "Division resulted in NaN!"
        );
    }
}

#[cfg(test)]
mod test_construction {
    use approx::assert_relative_eq;

    use crate::math::{PatinaFloat, tuples::tuple2::Tuple2};

    #[test]
    fn test_tuples_3() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 1.0);
    }

    #[test]
    fn test_tuples_3_zero() {
        let t = Tuple2::<PatinaFloat>::zero();
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 0.0);
    }
}

#[cfg(test)]
mod test_operations {
    use crate::math::{PatinaFloat, tuples::tuple2::Tuple2};

    #[test]
    fn test_addition() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.5, 3.0);

        assert_eq!(t + r, Tuple2::<PatinaFloat>::new(1.5, 4.0));
    }

    #[test]
    fn test_additive_identity() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let o = Tuple2::<PatinaFloat>::zero();

        assert_eq!(t + o, t);
    }

    #[test]
    fn test_add_assign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.5, 3.0);
        t += r;

        assert_eq!(t, Tuple2::<PatinaFloat>::new(1.5, 4.0));
    }

    #[test]
    fn test_negation() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);

        assert_eq!(-t, Tuple2::<PatinaFloat>::new(0.0, -1.0));
    }

    #[test]
    fn test_subtraction() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.5, 3.0);

        assert_eq!(t - r, Tuple2::<PatinaFloat>::new(-1.5, -2.0));
    }

    #[test]
    fn test_subassign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        t -= t;

        assert_eq!(t, Tuple2::<PatinaFloat>::zero());
    }

    #[test]
    fn test_scalar_mult() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        assert_eq!(t * 2.0, Tuple2::<PatinaFloat>::new(0.0, 2.0));
    }

    #[test]
    fn test_scalar_mult_assign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        t *= 2.0;
        assert_eq!(t, Tuple2::<PatinaFloat>::new(0.0, 2.0));
    }

    #[test]
    fn test_division() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);

        assert_eq!(t / 2.0, Tuple2::<PatinaFloat>::new(0.0, 0.5));
    }

    #[test]
    fn test_div_assign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        t /= 2.0;
        assert_eq!(t, Tuple2::<PatinaFloat>::new(0.0, 0.5));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero_panics() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let _ = t / 0.0;
    }

    #[test]
    #[should_panic]
    fn test_div_assign_by_zero_panics() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        t /= 0.0;
    }

    #[test]
    fn test_componentwise_mult() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.0, 4.0);

        assert_eq!(t * r, Tuple2::<PatinaFloat>::new(0.0, 4.0));
    }

    #[test]
    fn test_componentwise_mult_assign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.0, 4.0);

        t *= r;
        assert_eq!(t, Tuple2::<PatinaFloat>::new(0.0, 4.0));
    }

    #[test]
    fn test_componentwise_div() {
        let t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.0, 4.0);

        assert_eq!(t / r, Tuple2::<PatinaFloat>::new(0.0, 0.25));
    }
    #[test]
    fn test_componentwise_div_assign() {
        let mut t = Tuple2::<PatinaFloat>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat>::new(1.0, 4.0);
        t /= r;
        assert_eq!(t, Tuple2::<PatinaFloat>::new(0.0, 0.25));
    }
}

#[cfg(test)]
mod test_tuple3_functions {
    use crate::math::{PatinaFloat, tuples::tuple2::Tuple2};

    #[test]
    fn test_absolute_value() {
        let t = Tuple2::<PatinaFloat>::new(-1.3, 1.0);
        assert_eq!(t.abs(), Tuple2::<PatinaFloat>::new(1.3, 1.0));
    }

    #[test]
    fn test_ceil_and_floor() {
        let t = Tuple2::<PatinaFloat>::new(-1.3, 1.1);
        assert_eq!(t.ceil(), Tuple2::<PatinaFloat>::new(-1.0, 2.0));
        assert_eq!(t.floor(), Tuple2::<PatinaFloat>::new(-2.0, 1.0));
    }

    #[test]
    fn test_lerp() {
        let a = Tuple2::<PatinaFloat>::zero();
        let b = Tuple2::<PatinaFloat>::new(1.0, 1.0);

        assert_eq!(Tuple2::lerp(0.0, a, b), a);
        assert_eq!(Tuple2::lerp(1.0, a, b), b);
        assert_eq!(
            Tuple2::lerp(0.5, a, b),
            Tuple2::<PatinaFloat>::new(0.5, 0.5)
        );
    }

    #[test]
    fn test_min_max() {
        let a = Tuple2::<PatinaFloat>::new(-15.0, 999.0);
        let b = Tuple2::<PatinaFloat>::new(-14.0, 810.0);

        assert_eq!(Tuple2::max(a, b), Tuple2::<PatinaFloat>::new(-14.0, 999.0));
        assert_eq!(Tuple2::min(a, b), Tuple2::<PatinaFloat>::new(-15.0, 810.0));
    }

    #[test]
    fn test_comp_min_max() {
        let a = Tuple2::<PatinaFloat>::new(-15.0, 999.0);
        assert_eq!(a.min_component(), -15.0);
        assert_eq!(a.max_component(), 999.0);
    }

    #[test]
    fn test_comp_min_max_index() {
        let a = Tuple2::<PatinaFloat>::new(-15.0, 999.0);
        assert_eq!(a.min_component_index(), 0);
        assert_eq!(a.max_component_index(), 1);
    }

    #[test]
    fn test_permute() {
        let a = Tuple2::<PatinaFloat>::new(-15.0, 999.0);
        assert_eq!(a.permute(&[1, 0]), Tuple2::<PatinaFloat>::new(999.0, -15.0));
    }

    #[test]
    fn test_hprod() {
        let a = Tuple2::<PatinaFloat>::new(-10.0, 2.0);
        assert_eq!(a.hprod(), -20.0);
    }
}
