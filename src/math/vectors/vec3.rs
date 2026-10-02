use approx::relative_eq;
use num_traits::{Float, Num, ToPrimitive};
use std::f64::consts::PI;

use crate::math::{
    PatinaFloat, safe_asin,
    traits::CheckNan,
    tuples::{TupleLength, TypeVector, tuple3::Tuple3},
};

pub type Vector3<T> = Tuple3<T, TypeVector>;

impl<T> Vector3<T>
where
    T: Num + Copy + CheckNan + ToPrimitive,
{
    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> TupleLength {
        self.len_squared()
            .to_f64()
            .expect("Conversion to f64 failed")
            .sqrt()
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

    pub fn angle_between(&self, rhs: &Self) -> PatinaFloat {
        let u = self.normalize();
        let v = rhs.normalize();
        if u.dot(&v) < 0.0 {
            PI - 2.0 * safe_asin((u + v).len() / 2.0)
        } else {
            2.0 * safe_asin((v - u).len() / 2.0)
        }
    }

    // Given u (self) and a normalized vector w, compute a vector v from u that is orthogonal to w.
    pub fn gram_schmidt(self, w: Self) -> Self {
        let len = w.len();
        debug_assert!(
            approx::relative_eq!(len, 1.0),
            "vector not normalized in call to gram_schmidt: got length {len:?}"
        );
        self - w * self.dot(&w)
    }
}
impl<T: Float + CheckNan> Vector3<T> {
    // Given a vector normalized vector u (self), compute a local coordinate system such that the
    // three vectors are mutually perpendicular. We use the numerically stable method by Frisvad &
    // Duff: https://jcgt.org/published/0006/01/01/.
    pub fn coordinate_system(self) -> (Self, Self, Self) {
        debug_assert!(relative_eq!(self.len(), 1.0));
        let (x, y, z) = (self.x, self.y, self.z);
        let s = T::one().copysign(z);
        let a = -T::one() / (s + z);
        let b = x * y * a;

        (
            self,
            Vector3::new(T::one() + s * x * x * a, s * b, -s * x),
            Vector3::new(b, s + y * y * a, -y),
        )
    }
}

#[cfg(test)]
mod test_construction {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;
    use num_traits::Float;

    use crate::math::{tuples::TupleLength, vectors::PatinaVec3f};

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

    #[test]
    fn test_gram_schmidt() {
        let u = PatinaVec3f::new(1.0, 1.2, -3.2);
        let v = PatinaVec3f::new(1.2, 0.0, 3.2).normalize();

        let w = u.gram_schmidt(v);
        assert_relative_eq!(v.dot(&w), 0.0);
    }

    #[test]
    fn test_coordinate_system() {
        let u = PatinaVec3f::new(1.0, 1.2, -3.2);
        let (e1, e2, e3) = u.normalize().coordinate_system();

        assert_relative_eq!(e1.dot(&e2), 0.0);
        assert_relative_eq!(e1.dot(&e3), 0.0);
        assert_relative_eq!(e2.dot(&e3), 0.0);
    }
}
