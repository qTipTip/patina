use std::ops::{self, AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg, SubAssign};

use num_traits::{Float, Num, float::FloatCore};

pub trait CheckNan {
    fn is_nan_val(&self) -> bool;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub(crate) struct Tuple3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
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

impl<T: Num + CheckNan + Copy> Tuple3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(
            !x.is_nan_val() && !y.is_nan_val() && !z.is_nan_val(),
            "Tuple3 components cannot be NaN!"
        );
        Self { x, y, z }
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

impl<T: Num + CheckNan + Copy + PartialOrd> Tuple3<T> {
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

impl<T: num_traits::Signed + CheckNan + Copy> Tuple3<T> {
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl<T: Num + FloatCore + CheckNan + Copy> Tuple3<T> {
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl<T: Float + CheckNan> Tuple3<T> {
    pub fn fma(a: Self, b: Self, c: Self) -> Self {
        a * b + c
    }
}

impl<T> Index<usize> for Tuple3<T> {
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

impl<T> IndexMut<usize> for Tuple3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Indexing into item with length 3, with index {index}"),
        }
    }
}

impl<T: Num + CheckNan + Copy> ops::Add for Tuple3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign> ops::AddAssign for Tuple3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy> ops::Neg for Tuple3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Num + Neg<Output = T> + CheckNan + Copy> ops::Sub for Tuple3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: SubAssign> ops::SubAssign for Tuple3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Num + CheckNan + Copy> ops::Mul<T> for Tuple3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: MulAssign + Copy> ops::MulAssign<T> for Tuple3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Num + CheckNan + Copy> ops::Mul<Tuple3<T>> for Tuple3<T> {
    type Output = Self;
    fn mul(self, rhs: Tuple3<T>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: MulAssign> ops::MulAssign<Tuple3<T>> for Tuple3<T> {
    fn mul_assign(&mut self, rhs: Tuple3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Num + CheckNan + Copy> ops::Div<Tuple3<T>> for Tuple3<T> {
    type Output = Self;
    fn div(self, rhs: Tuple3<T>) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: DivAssign> ops::DivAssign<Tuple3<T>> for Tuple3<T> {
    fn div_assign(&mut self, rhs: Tuple3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: Num + CheckNan + Copy> ops::Div<T> for Tuple3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: DivAssign + Copy> ops::DivAssign<T> for Tuple3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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

    #[test]
    fn test_lerp() {
        let a = Tuple3::<PatinaFloat>::zero();
        let b = Tuple3::<PatinaFloat>::new(1.0, 1.0, 1.0);

        assert_eq!(Tuple3::lerp(0.0, a, b), a);
        assert_eq!(Tuple3::lerp(1.0, a, b), b);
        assert_eq!(
            Tuple3::lerp(0.5, a, b),
            Tuple3::<PatinaFloat>::new(0.5, 0.5, 0.5)
        );
    }

    #[test]
    fn test_min_max() {
        let a = Tuple3::<PatinaFloat>::new(-15.0, 999.0, 1.0);
        let b = Tuple3::<PatinaFloat>::new(-14.0, 810.0, 3.0);

        assert_eq!(
            Tuple3::max(a, b),
            Tuple3::<PatinaFloat>::new(-14.0, 999.0, 3.0)
        );
        assert_eq!(
            Tuple3::min(a, b),
            Tuple3::<PatinaFloat>::new(-15.0, 810.0, 1.0)
        );
    }

    #[test]
    fn test_comp_min_max() {
        let a = Tuple3::<PatinaFloat>::new(-15.0, 999.0, 1.0);
        assert_eq!(a.min_component(), -15.0);
        assert_eq!(a.max_component(), 999.0);
    }

    #[test]
    fn test_comp_min_max_index() {
        let a = Tuple3::<PatinaFloat>::new(-15.0, 999.0, 1.0);
        assert_eq!(a.min_component_index(), 0);
        assert_eq!(a.max_component_index(), 1);
    }

    #[test]
    fn test_permute() {
        let a = Tuple3::<PatinaFloat>::new(-15.0, 999.0, 1.0);
        assert_eq!(
            a.permute(&[1, 2, 0]),
            Tuple3::<PatinaFloat>::new(999.0, 1.0, -15.0)
        );
    }

    #[test]
    fn test_hprod() {
        let a = Tuple3::<PatinaFloat>::new(-10.0, 2.0, 1.0);
        assert_eq!(a.hprod(), -20.0);
    }
}

#[cfg(test)]
mod test_integer_tuples {
    use crate::math::{PatinaInt, tuples::tuple3::Tuple3};

    #[test]
    fn test_integer_tuples_work() {
        let t = Tuple3::<PatinaInt>::new(-10, 2, 1);
        let r = Tuple3::<PatinaInt>::new(-10, 2, 5);

        assert_eq!(t * r, Tuple3::<PatinaInt>::new(100, 4, 5));
    }
}
