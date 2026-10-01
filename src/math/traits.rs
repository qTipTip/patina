use std::{
    ops::{Add, Deref, Mul},
    process::Output,
};

use crate::math::tuples::{tuple2::Tuple2, tuple3::Tuple3};

// We use the CheckNan-trait in combination with IsNotFloat-trait to provide a default `is_nan`
// check on our Tuple-types.
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

// Direction: The Direction-trait lets us implement dot-product for any combination of `Normal`s and
// `Vecs`.
pub trait Direction2<T>: Deref<Target = Tuple2<T>> {
    fn dot<R: Direction3<T>>(&self, rhs: &R) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y
    }
}
pub trait Direction3<T>: Deref<Target = Tuple3<T>> {
    fn dot<R: Direction3<T>>(&self, rhs: &R) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
