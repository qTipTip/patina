use std::{
    f64::consts::PI,
    ops::{Add, Deref, DerefMut, Sub},
};

use num_traits::{Num, ToPrimitive};

use crate::{
    implement_geometry_ops,
    math::{
        PatinaFloat, PatinaInt, safe_asin,
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

impl<T> Vector3<T>
where
    T: Num + Copy + CheckNan + ToPrimitive,
{
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

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // TODO: Compute this using a difference of products (using FMA), as it's more numerically
        // stable, even at f32.
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    // normalize cannot return Vector3<T>, as an integer vector normalized requires float values.
    pub fn normalize(&self) -> Vector3<TupleLength> {
        let len = self.len();
        if len == 0.0 {
            return Vector3::<TupleLength>::new(0.0, 0.0, 0.0);
        }
        Vector3::<TupleLength>::new(
            self.x.to_f64().expect("Conversion to f64 failed") / len,
            self.y.to_f64().expect("Conversion to f64 failed") / len,
            self.z.to_f64().expect("Conversion to f64 failed") / len,
        )
    }
}
impl<T> Vector3<T>
where
    T: Num + Copy + CheckNan + ToPrimitive + PartialOrd,
    for<'a> &'a Vector3<T>: Add<&'a Vector3<T>, Output = Vector3<T>>,
    for<'a> &'a Vector3<T>: Sub<&'a Vector3<T>, Output = Vector3<T>>,
{
    pub fn angle_between(&self, rhs: &Self) -> PatinaFloat {
        let u = self.normalize();
        let v = rhs.normalize();
        if u.dot(&v) < 0.0 {
            PI - 2.0 * safe_asin((&u + &v).len() / 2.0)
        } else {
            2.0 * safe_asin((&v - &u).len() / 2.0)
        }
    }
}

#[cfg(test)]
mod test_construction {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;
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

    #[test]
    fn test_angle_between() {
        let e1 = PatinaVec3f::new(1.0, 0.0, 0.0);
        let e2 = PatinaVec3f::new(0.0, 1.0, 0.0);

        assert_relative_eq!(e1.angle_between(&e2), PI / 2.0);
    }

    #[test]
    fn test_cross_product() {
        let e1 = PatinaVec3f::new(1.0, 0.0, 0.0);
        let e2 = PatinaVec3f::new(0.0, 1.0, 0.0);

        assert_eq!(e1.cross(&e2), PatinaVec3f::new(0.0, 0.0, 1.0));
    }
}
