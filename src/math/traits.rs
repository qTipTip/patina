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
