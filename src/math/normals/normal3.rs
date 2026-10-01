use std::ops::{Deref, DerefMut};

use crate::math::tuples::tuple3::Tuple3;

struct Normal3<T>(Tuple3<T>);

impl<T> DerefMut for Normal3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for Normal3<T> {
    type Target = Tuple3<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
