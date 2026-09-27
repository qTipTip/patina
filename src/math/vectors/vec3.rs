use num_traits::Num;

use crate::math::{
    PatinaFloat, PatinaInt,
    tuples::tuple3::{CheckNan, Tuple3},
};

struct Vector3<T>(Tuple3<T>);

type PatinaVec3f = Vector3<PatinaFloat>;
type PatinaVec3i = Vector3<PatinaInt>;

impl<T: Num + Copy + CheckNan> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple3::<T>::new(x, y, z))
    }
}
