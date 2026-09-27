use std::ops::{Deref, DerefMut};

use num_traits::Num;

use crate::math::{
    PatinaFloat, PatinaInt,
    tuples::tuple3::{CheckNan, Tuple3},
};

struct Vector3<T>(Tuple3<T>);

type PatinaVec3f = Vector3<PatinaFloat>;
type PatinaVec3i = Vector3<PatinaInt>;

impl<T> Deref for Vector3<T> {
    type Target = Tuple3<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Num + Copy + CheckNan> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple3::<T>::new(x, y, z))
    }
}

#[cfg(test)]
mod test_construction {
    use crate::math::vectors::vec3::PatinaVec3f;

    #[test]
    fn test_vec3_constructor() {
        let v = PatinaVec3f::new(0.0, 1.0, 2.0);
    }
}
