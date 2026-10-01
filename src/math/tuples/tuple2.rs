use std::{
    marker::PhantomData,
    ops::{self, AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg, SubAssign},
};

use num_traits::{Float, Num, float::FloatCore};

use crate::math::{
    traits::CheckNan,
    tuples::{Difference, Direction},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tuple2<T, Type> {
    pub x: T,
    pub y: T,
    _type: PhantomData<Type>,
}

impl<T: Num + CheckNan + Copy, Type> Tuple2<T, Type> {
    pub fn new(x: T, y: T) -> Self {
        debug_assert!(
            !x.is_nan_val() && !y.is_nan_val(),
            "Tuple2 components cannot be NaN!"
        );
        Self {
            x,
            y,
            _type: PhantomData,
        }
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

impl<T: Num + CheckNan + Copy + PartialOrd, Type> Tuple2<T, Type> {
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

impl<T: num_traits::Signed + CheckNan + Copy, Type> Tuple2<T, Type> {
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl<T: Num + FloatCore + CheckNan + Copy, Type> Tuple2<T, Type> {
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Float + CheckNan, Type> Tuple2<T, Type> {
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn fma(a: Self, b: Self, c: Self) -> Self {
        a * b + c
    }
}

// Dot product between any two directions: vector or normal, in either order.
impl<T: Num + Copy, Type: Direction> Tuple2<T, Type> {
    pub fn dot<Rhs: Direction>(&self, rhs: &Tuple2<T, Rhs>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T, Type> Index<usize> for Tuple2<T, Type> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Indexing into item with length 2, with index {index}"),
        }
    }
}

impl<T, Type> IndexMut<usize> for Tuple2<T, Type> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Indexing into item with length 2, with index {index}"),
        }
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Add for Tuple2<T, Type> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign, Type> ops::AddAssign for Tuple2<T, Type> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy, Type> ops::Neg for Tuple2<T, Type> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

// The tag decides what a difference is: Point - Point = Vector.
impl<T: Num + CheckNan + Copy, Type: Difference> ops::Sub for Tuple2<T, Type> {
    type Output = Tuple2<T, Type::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign, Type: Difference<Output = Type>> ops::SubAssign for Tuple2<T, Type> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Mul<T> for Tuple2<T, Type> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: MulAssign + Copy + CheckNan, Type> ops::MulAssign<T> for Tuple2<T, Type> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val(),
            "Multiplication resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Mul<Tuple2<T, Type>> for Tuple2<T, Type> {
    type Output = Self;
    fn mul(self, rhs: Tuple2<T, Type>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: MulAssign + CheckNan, Type> ops::MulAssign<Tuple2<T, Type>> for Tuple2<T, Type> {
    fn mul_assign(&mut self, rhs: Tuple2<T, Type>) {
        self.x *= rhs.x;
        self.y *= rhs.y;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val(),
            "Multiplication resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Div<Tuple2<T, Type>> for Tuple2<T, Type> {
    type Output = Self;
    fn div(self, rhs: Tuple2<T, Type>) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: DivAssign + CheckNan, Type> ops::DivAssign<Tuple2<T, Type>> for Tuple2<T, Type> {
    fn div_assign(&mut self, rhs: Tuple2<T, Type>) {
        self.x /= rhs.x;
        self.y /= rhs.y;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val(),
            "Division resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Div<T> for Tuple2<T, Type> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: DivAssign + Copy + CheckNan, Type> ops::DivAssign<T> for Tuple2<T, Type> {
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

    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple2::Tuple2},
    };

    #[test]
    fn test_tuples_3() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 1.0);
    }

    #[test]
    fn test_tuples_3_zero() {
        let t = Tuple2::<PatinaFloat, TypeVector>::zero();
        assert!(!t.has_nan());

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 0.0);
    }
}

#[cfg(test)]
mod test_operations {
    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple2::Tuple2},
    };

    #[test]
    fn test_addition() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.5, 3.0);

        assert_eq!(t + r, Tuple2::<PatinaFloat, TypeVector>::new(1.5, 4.0));
    }

    #[test]
    fn test_additive_identity() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let o = Tuple2::<PatinaFloat, TypeVector>::zero();

        assert_eq!(t + o, t);
    }

    #[test]
    fn test_add_assign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.5, 3.0);
        t += r;

        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::new(1.5, 4.0));
    }

    #[test]
    fn test_negation() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);

        assert_eq!(-t, Tuple2::<PatinaFloat, TypeVector>::new(0.0, -1.0));
    }

    #[test]
    fn test_subtraction() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.5, 3.0);

        assert_eq!(t - r, Tuple2::<PatinaFloat, TypeVector>::new(-1.5, -2.0));
    }

    #[test]
    fn test_subassign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        t -= t;

        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::zero());
    }

    #[test]
    fn test_scalar_mult() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        assert_eq!(t * 2.0, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 2.0));
    }

    #[test]
    fn test_scalar_mult_assign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        t *= 2.0;
        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 2.0));
    }

    #[test]
    fn test_division() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);

        assert_eq!(t / 2.0, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 0.5));
    }

    #[test]
    fn test_div_assign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        t /= 2.0;
        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 0.5));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero_panics() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let _ = t / 0.0;
    }

    #[test]
    #[should_panic]
    fn test_div_assign_by_zero_panics() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        t /= 0.0;
    }

    #[test]
    fn test_componentwise_mult() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.0, 4.0);

        assert_eq!(t * r, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 4.0));
    }

    #[test]
    fn test_componentwise_mult_assign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.0, 4.0);

        t *= r;
        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 4.0));
    }

    #[test]
    fn test_componentwise_div() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.0, 4.0);

        assert_eq!(t / r, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 0.25));
    }
    #[test]
    fn test_componentwise_div_assign() {
        let mut t = Tuple2::<PatinaFloat, TypeVector>::new(0.0, 1.0);
        let r = Tuple2::<PatinaFloat, TypeVector>::new(1.0, 4.0);
        t /= r;
        assert_eq!(t, Tuple2::<PatinaFloat, TypeVector>::new(0.0, 0.25));
    }
}

#[cfg(test)]
mod test_tuple3_functions {
    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple2::Tuple2},
    };

    #[test]
    fn test_absolute_value() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(-1.3, 1.0);
        assert_eq!(t.abs(), Tuple2::<PatinaFloat, TypeVector>::new(1.3, 1.0));
    }

    #[test]
    fn test_ceil_and_floor() {
        let t = Tuple2::<PatinaFloat, TypeVector>::new(-1.3, 1.1);
        assert_eq!(t.ceil(), Tuple2::<PatinaFloat, TypeVector>::new(-1.0, 2.0));
        assert_eq!(t.floor(), Tuple2::<PatinaFloat, TypeVector>::new(-2.0, 1.0));
    }

    #[test]
    fn test_lerp() {
        let a = Tuple2::<PatinaFloat, TypeVector>::zero();
        let b = Tuple2::<PatinaFloat, TypeVector>::new(1.0, 1.0);

        assert_eq!(Tuple2::lerp(0.0, a, b), a);
        assert_eq!(Tuple2::lerp(1.0, a, b), b);
        assert_eq!(
            Tuple2::lerp(0.5, a, b),
            Tuple2::<PatinaFloat, TypeVector>::new(0.5, 0.5)
        );
    }

    #[test]
    fn test_min_max() {
        let a = Tuple2::<PatinaFloat, TypeVector>::new(-15.0, 999.0);
        let b = Tuple2::<PatinaFloat, TypeVector>::new(-14.0, 810.0);

        assert_eq!(
            Tuple2::max(a, b),
            Tuple2::<PatinaFloat, TypeVector>::new(-14.0, 999.0)
        );
        assert_eq!(
            Tuple2::min(a, b),
            Tuple2::<PatinaFloat, TypeVector>::new(-15.0, 810.0)
        );
    }

    #[test]
    fn test_comp_min_max() {
        let a = Tuple2::<PatinaFloat, TypeVector>::new(-15.0, 999.0);
        assert_eq!(a.min_component(), -15.0);
        assert_eq!(a.max_component(), 999.0);
    }

    #[test]
    fn test_comp_min_max_index() {
        let a = Tuple2::<PatinaFloat, TypeVector>::new(-15.0, 999.0);
        assert_eq!(a.min_component_index(), 0);
        assert_eq!(a.max_component_index(), 1);
    }

    #[test]
    fn test_permute() {
        let a = Tuple2::<PatinaFloat, TypeVector>::new(-15.0, 999.0);
        assert_eq!(
            a.permute(&[1, 0]),
            Tuple2::<PatinaFloat, TypeVector>::new(999.0, -15.0)
        );
    }

    #[test]
    fn test_hprod() {
        let a = Tuple2::<PatinaFloat, TypeVector>::new(-10.0, 2.0);
        assert_eq!(a.hprod(), -20.0);
    }
}
