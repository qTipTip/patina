use crate::math::{
    traits::{CheckNan, Direction3},
    vectors::vec3::Vector3,
};
use num_traits::Num;
use std::ops::{Deref, DerefMut};

use crate::{implement_geometry_ops, math::tuples::tuple3::Tuple3};

struct Normal3<T>(Tuple3<T>);

implement_geometry_ops!(Normal3, Tuple3, [Add, Sub, Mul, Neg]);

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
impl<T: Num + Copy + CheckNan> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple3::new(x, y, z))
    }
}

impl<T: Num + Copy + CheckNan> From<Vector3<T>> for Normal3<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl<T> Direction3<T> for Normal3<T> {}
