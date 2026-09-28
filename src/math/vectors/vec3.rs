use std::ops::{Deref, DerefMut};

use num_traits::{Num, ToPrimitive};

use crate::{
    implement_geometry_ops,
    math::{
        PatinaFloat, PatinaInt,
        traits::CheckNan,
        tuples::{TupleLength, tuple3::Tuple3},
    },
};

#[derive(PartialEq, Debug)]
struct Vector3<T>(Tuple3<T>);

implement_geometry_ops!(Vector3, Tuple3);

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

impl<T: Num + Copy + CheckNan + ToPrimitive> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple3::<T>::new(x, y, z))
    }

    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> TupleLength {
        self.len_squared()
            .to_f64()
            .expect("Conversion to f64 failed")
            .sqrt()
    }

    // normalize cannot return Vector3<T>, as an integer vector normalized requires float values.
    pub fn normalize(&self) -> Vector3<TupleLength> {
        let len = self.len();
        if len == 0.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        Vector3::<TupleLength>::new(
            self.x.to_f64().expect("Conversion to f64 failed") / len,
            self.y.to_f64().expect("Conversion to f64 failed") / len,
            self.z.to_f64().expect("Conversion to f64 failed") / len,
        )
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn angle_between(&self, rhs: &Self) -> PatinaFloat {
        if self.dot(rhs) < T::zero() {
            return;
        }
    }
}

#[cfg(test)]
mod test_construction {
    use num_traits::Float;

    use crate::math::{tuples::TupleLength, vectors::vec3::PatinaVec3f};

    #[test]
    fn test_vec3_constructor() {
        let t = PatinaVec3f::new(0.0, 1.0, 2.0);
        let r = PatinaVec3f::new(1.0, 2.0, 3.0);

        assert_eq!(t + r, PatinaVec3f::new(1.0, 3.0, 5.0));
    }

    #[test]
    fn test_vec3_length_squared() {
        let t = PatinaVec3f::new(0.0, 1.0, 2.0);
        assert_eq!(t.len_squared(), 5.0);
    }

    #[test]
    fn test_vec3_length() {
        let t = PatinaVec3f::new(0.0, 1.0, 2.0);
        assert_eq!(t.len(), TupleLength::sqrt(5.0));
    }

    #[test]
    fn test_vec3_normalize() {
        let t = PatinaVec3f::new(0.0, 1.0, 2.0);
        assert_eq!(
            t.normalize(),
            PatinaVec3f::new(0.0, 1.0 / 5.0.sqrt(), 2.0 / 5.0.sqrt())
        );
    }
}
