use std::ops::{Index, IndexMut};

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

impl Index for Tuple3<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        if index == 0 {
            return self.x;
        };
        if index == 1 {
            return self.y;
        };
        if index == 2 {
            return self.z;
        }
        panic!("Indexing into item with length 3, with index {index}")
    }
}

impl IndexMut for Tuple3<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
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
