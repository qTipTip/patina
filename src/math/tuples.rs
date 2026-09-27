use num_traits::Float;

#[derive(Clone, Copy)]
pub(crate) struct Tuple2<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub(crate) struct Tuple3<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Tuple3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let t = Self { x, y, z };
        debug_assert!(!t.has_nan());
        t
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn has_nan() -> bool {
        Float::is_nan(x) || Float::is_nan(y) || Float::is_nan(z)
    }
}
