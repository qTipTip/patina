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
pub struct Tuple3<T, Type> {
    pub x: T,
    pub y: T,
    pub z: T,
    _type: PhantomData<Type>,
}

impl<T: Num + CheckNan + Copy, Type> Tuple3<T, Type> {
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(
            !x.is_nan_val() && !y.is_nan_val() && !z.is_nan_val(),
            "Tuple3 components cannot be NaN!"
        );
        Self {
            x,
            y,
            z,
            _type: PhantomData,
        }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn lerp(t: T, a: Self, b: Self) -> Self {
        a * (T::one() - t) + b * t
    }

    pub fn hprod(&self) -> T {
        self.x * self.y * self.z
    }

    pub fn permute(&self, perm_indices: &[usize; 3]) -> Self {
        Self::new(
            self[perm_indices[0]],
            self[perm_indices[1]],
            self[perm_indices[2]],
        )
    }
}

impl<T: Num + CheckNan + Copy + PartialOrd, Type> Tuple3<T, Type> {
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(
            if a.x <= b.x { a.x } else { b.x },
            if a.y <= b.y { a.y } else { b.y },
            if a.z <= b.z { a.z } else { b.z },
        )
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self::new(
            if a.x >= b.x { a.x } else { b.x },
            if a.y >= b.y { a.y } else { b.y },
            if a.z >= b.z { a.z } else { b.z },
        )
    }

    pub fn min_component(&self) -> T {
        let mut min = self.x;
        if self.y < min {
            min = self.y;
        }
        if self.z < min {
            min = self.z;
        }
        min
    }

    pub fn max_component(&self) -> T {
        let mut max = self.x;
        if self.y > max {
            max = self.y;
        }
        if self.z > max {
            max = self.z;
        }
        max
    }

    pub fn min_component_index(&self) -> usize {
        if self.x <= self.y && self.x <= self.z {
            0
        } else if self.y <= self.z {
            1
        } else {
            2
        }
    }

    pub fn max_component_index(&self) -> usize {
        if self.x >= self.y && self.x >= self.z {
            0
        } else if self.y >= self.z {
            1
        } else {
            2
        }
    }
}

impl<T: num_traits::Signed + CheckNan + Copy, Type> Tuple3<T, Type> {
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl<T: Num + FloatCore + CheckNan + Copy, Type> Tuple3<T, Type> {
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl<T: Float + CheckNan, Type> Tuple3<T, Type> {
    pub fn fma(a: Self, b: Self, c: Self) -> Self {
        a * b + c
    }
}

// Dot product between any two directions: vector or normal, in either order.
impl<T: Num + Copy, Type: Direction> Tuple3<T, Type> {
    pub fn dot<Rhs: Direction>(&self, rhs: &Tuple3<T, Rhs>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn face_forward<Rhs: Direction>(self, rhs: Tuple3<T, Rhs>) -> Self
    where
        T: PartialOrd + Neg<Output = T> + CheckNan,
    {
        if self.dot(&rhs) < T::zero() {
            -self
        } else {
            self
        }
    }
}

impl<T, Type> Index<usize> for Tuple3<T, Type> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Indexing into item with length 3, with index {index}"),
        }
    }
}

impl<T, Type> IndexMut<usize> for Tuple3<T, Type> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Indexing into item with length 3, with index {index}"),
        }
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Add for Tuple3<T, Type> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign, Type> ops::AddAssign for Tuple3<T, Type> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy, Type> ops::Neg for Tuple3<T, Type> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// The tag decides what a difference is: Point - Point = Vector.
impl<T: Num + CheckNan + Copy, Type: Difference> ops::Sub for Tuple3<T, Type> {
    type Output = Tuple3<T, Type::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign, Type: Difference<Output = Type>> ops::SubAssign for Tuple3<T, Type> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Mul<T> for Tuple3<T, Type> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: MulAssign + Copy + CheckNan, Type> ops::MulAssign<T> for Tuple3<T, Type> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val() && !self.z.is_nan_val(),
            "Multiplication resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Mul<Tuple3<T, Type>> for Tuple3<T, Type> {
    type Output = Self;
    fn mul(self, rhs: Tuple3<T, Type>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: MulAssign + CheckNan, Type> ops::MulAssign<Tuple3<T, Type>> for Tuple3<T, Type> {
    fn mul_assign(&mut self, rhs: Tuple3<T, Type>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val() && !self.z.is_nan_val(),
            "Multiplication resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Div<Tuple3<T, Type>> for Tuple3<T, Type> {
    type Output = Self;
    fn div(self, rhs: Tuple3<T, Type>) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: DivAssign + CheckNan, Type> ops::DivAssign<Tuple3<T, Type>> for Tuple3<T, Type> {
    fn div_assign(&mut self, rhs: Tuple3<T, Type>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;

        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val() && !self.z.is_nan_val(),
            "Division resulted in NaN!"
        );
    }
}

impl<T: Num + CheckNan + Copy, Type> ops::Div<T> for Tuple3<T, Type> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: DivAssign + Copy + CheckNan, Type> ops::DivAssign<T> for Tuple3<T, Type> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        debug_assert!(
            !self.x.is_nan_val() && !self.y.is_nan_val() && !self.z.is_nan_val(),
            "Division resulted in NaN!"
        );
    }
}

#[cfg(test)]
mod test_construction {
    use approx::assert_relative_eq;

    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple3::Tuple3},
    };

    #[test]
    fn test_tuples_3() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 1.0);
        assert_relative_eq!(t[2], 2.0);
    }

    #[test]
    fn test_tuples_3_zero() {
        let t = Tuple3::<PatinaFloat, TypeVector>::zero();

        assert_relative_eq!(t[0], 0.0);
        assert_relative_eq!(t[1], 0.0);
        assert_relative_eq!(t[2], 0.0);
    }
}

#[cfg(test)]
mod test_operations {
    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple3::Tuple3},
    };

    #[test]
    fn test_addition() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.5, 3.0, 2.0);

        assert_eq!(t + r, Tuple3::<PatinaFloat, TypeVector>::new(1.5, 4.0, 4.0));
    }

    #[test]
    fn test_additive_identity() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let o = Tuple3::<PatinaFloat, TypeVector>::zero();

        assert_eq!(t + o, t);
    }

    #[test]
    fn test_add_assign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.5, 3.0, 2.0);
        t += r;

        assert_eq!(t, Tuple3::<PatinaFloat, TypeVector>::new(1.5, 4.0, 4.0));
    }

    #[test]
    fn test_negation() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);

        assert_eq!(-t, Tuple3::<PatinaFloat, TypeVector>::new(0.0, -1.0, -2.0));
    }

    #[test]
    fn test_subtraction() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.5, 3.0, 2.0);

        assert_eq!(
            t - r,
            Tuple3::<PatinaFloat, TypeVector>::new(-1.5, -2.0, 0.0)
        );
    }

    #[test]
    fn test_subassign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        t -= t;

        assert_eq!(t, Tuple3::<PatinaFloat, TypeVector>::zero());
    }

    #[test]
    fn test_scalar_mult() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        assert_eq!(
            t * 2.0,
            Tuple3::<PatinaFloat, TypeVector>::new(0.0, 2.0, 4.0)
        );
    }

    #[test]
    fn test_scalar_mult_assign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        t *= 2.0;
        assert_eq!(t, Tuple3::<PatinaFloat, TypeVector>::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn test_division() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);

        assert_eq!(
            t / 2.0,
            Tuple3::<PatinaFloat, TypeVector>::new(0.0, 0.5, 1.0)
        );
    }

    #[test]
    fn test_div_assign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        t /= 2.0;
        assert_eq!(t, Tuple3::<PatinaFloat, TypeVector>::new(0.0, 0.5, 1.0));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero_panics() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let _ = t / 0.0;
    }

    #[test]
    #[should_panic]
    fn test_div_assign_by_zero_panics() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        t /= 0.0;
    }

    #[test]
    fn test_componentwise_mult() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.0, 4.0, 3.5);

        assert_eq!(t * r, Tuple3::<PatinaFloat, TypeVector>::new(0.0, 4.0, 7.0));
    }

    #[test]
    fn test_componentwise_mult_assign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.0, 4.0, 3.5);

        t *= r;
        assert_eq!(t, Tuple3::<PatinaFloat, TypeVector>::new(0.0, 4.0, 7.0));
    }

    #[test]
    fn test_componentwise_div() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.0, 4.0, 3.0);

        assert_eq!(
            t / r,
            Tuple3::<PatinaFloat, TypeVector>::new(0.0, 0.25, 2.0 / 3.0)
        );
    }
    #[test]
    fn test_componentwise_div_assign() {
        let mut t = Tuple3::<PatinaFloat, TypeVector>::new(0.0, 1.0, 2.0);
        let r = Tuple3::<PatinaFloat, TypeVector>::new(1.0, 4.0, 3.0);
        t /= r;
        assert_eq!(
            t,
            Tuple3::<PatinaFloat, TypeVector>::new(0.0, 0.25, 2.0 / 3.0)
        );
    }
}

#[cfg(test)]
mod test_tuple3_functions {
    use crate::math::{
        PatinaFloat,
        tuples::{TypeVector, tuple3::Tuple3},
    };

    #[test]
    fn test_absolute_value() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(-1.3, 1.0, -12.0);
        assert_eq!(
            t.abs(),
            Tuple3::<PatinaFloat, TypeVector>::new(1.3, 1.0, 12.0)
        );
    }

    #[test]
    fn test_ceil_and_floor() {
        let t = Tuple3::<PatinaFloat, TypeVector>::new(-1.3, 1.1, 2.8);
        assert_eq!(
            t.ceil(),
            Tuple3::<PatinaFloat, TypeVector>::new(-1.0, 2.0, 3.0)
        );
        assert_eq!(
            t.floor(),
            Tuple3::<PatinaFloat, TypeVector>::new(-2.0, 1.0, 2.0)
        );
    }

    #[test]
    fn test_lerp() {
        let a = Tuple3::<PatinaFloat, TypeVector>::zero();
        let b = Tuple3::<PatinaFloat, TypeVector>::new(1.0, 1.0, 1.0);

        assert_eq!(Tuple3::lerp(0.0, a, b), a);
        assert_eq!(Tuple3::lerp(1.0, a, b), b);
        assert_eq!(
            Tuple3::lerp(0.5, a, b),
            Tuple3::<PatinaFloat, TypeVector>::new(0.5, 0.5, 0.5)
        );
    }

    #[test]
    fn test_min_max() {
        let a = Tuple3::<PatinaFloat, TypeVector>::new(-15.0, 999.0, 1.0);
        let b = Tuple3::<PatinaFloat, TypeVector>::new(-14.0, 810.0, 3.0);

        assert_eq!(
            Tuple3::max(a, b),
            Tuple3::<PatinaFloat, TypeVector>::new(-14.0, 999.0, 3.0)
        );
        assert_eq!(
            Tuple3::min(a, b),
            Tuple3::<PatinaFloat, TypeVector>::new(-15.0, 810.0, 1.0)
        );
    }

    #[test]
    fn test_comp_min_max() {
        let a = Tuple3::<PatinaFloat, TypeVector>::new(-15.0, 999.0, 1.0);
        assert_eq!(a.min_component(), -15.0);
        assert_eq!(a.max_component(), 999.0);
    }

    #[test]
    fn test_comp_min_max_index() {
        let a = Tuple3::<PatinaFloat, TypeVector>::new(-15.0, 999.0, 1.0);
        assert_eq!(a.min_component_index(), 0);
        assert_eq!(a.max_component_index(), 1);
    }

    #[test]
    fn test_permute() {
        let a = Tuple3::<PatinaFloat, TypeVector>::new(-15.0, 999.0, 1.0);
        assert_eq!(
            a.permute(&[1, 2, 0]),
            Tuple3::<PatinaFloat, TypeVector>::new(999.0, 1.0, -15.0)
        );
    }

    #[test]
    fn test_hprod() {
        let a = Tuple3::<PatinaFloat, TypeVector>::new(-10.0, 2.0, 1.0);
        assert_eq!(a.hprod(), -20.0);
    }
}

#[cfg(test)]
mod test_integer_tuples {
    use crate::math::{
        PatinaInt,
        tuples::{TypeVector, tuple3::Tuple3},
    };

    #[test]
    fn test_integer_tuples_work() {
        let t = Tuple3::<PatinaInt, TypeVector>::new(-10, 2, 1);
        let r = Tuple3::<PatinaInt, TypeVector>::new(-10, 2, 5);

        assert_eq!(t * r, Tuple3::<PatinaInt, TypeVector>::new(100, 4, 5));
    }
}
