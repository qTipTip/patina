use std::f64::consts::PI;

use crate::math::{
    PatinaFloat, PatinaInt, safe_asin,
    traits::CheckNan,
    tuples::{TupleLength, TypeVector, tuple2::Tuple2},
};
use num_traits::{Num, ToPrimitive};

pub type Vector2<T> = Tuple2<T, TypeVector>;

type PatinaVec2f = Vector2<PatinaFloat>;
type PatinaVec2i = Vector2<PatinaInt>;

impl<T> Vector2<T>
where
    T: Num + Copy + CheckNan + ToPrimitive,
{
    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> TupleLength {
        self.len_squared()
            .to_f64()
            .expect("Conversion to f64 failed")
            .sqrt()
    }

    pub fn cross(&self, rhs: &Self) -> T {
        // TODO: Compute this using a difference of products (using FMA), as it's more numerically
        // stable, even at f32.
        self.x * rhs.y - self.y * rhs.x
    }

    // normalize cannot return Vector2<T>, as an integer vector normalized requires float values.
    pub fn normalize(&self) -> Vector2<TupleLength> {
        let len = self.len();
        if len == 0.0 {
            return Vector2::<TupleLength>::new(0.0, 0.0);
        }
        Vector2::<TupleLength>::new(
            self.x.to_f64().expect("Conversion to f64 failed") / len,
            self.y.to_f64().expect("Conversion to f64 failed") / len,
        )
    }

    pub fn angle_between(&self, rhs: &Self) -> PatinaFloat {
        let u = self.normalize();
        let v = rhs.normalize();
        if u.dot(&v) < 0.0 {
            PI - 2.0 * safe_asin((u + v).len() / 2.0)
        } else {
            2.0 * {
                let x = (v - u).len() / 2.0;
                x.clamp(-1.0, 1.0).asin()
            }
        }
    }
}

#[cfg(test)]
mod test_construction {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;
    use num_traits::Float;

    use crate::math::{tuples::TupleLength, vectors::vec2::PatinaVec2f};

    #[test]
    fn test_vec2_constructor() {
        let t = PatinaVec2f::new(0.0, 1.0);
        let r = PatinaVec2f::new(1.0, 2.0);

        assert_eq!(t + r, PatinaVec2f::new(1.0, 3.0));
    }

    #[test]
    fn test_vec2_length_squared() {
        let t = PatinaVec2f::new(1.0, 1.0);
        assert_eq!(t.len_squared(), 2.0);
    }

    #[test]
    fn test_vec2_length() {
        let t = PatinaVec2f::new(1.0, 1.0);
        assert_eq!(t.len(), TupleLength::sqrt(2.0));
    }

    #[test]
    fn test_vec2_normalize() {
        let t = PatinaVec2f::new(1.0, 1.0);
        assert_eq!(
            t.normalize(),
            PatinaVec2f::new(1.0 / 2.0.sqrt(), 1.0 / 2.0.sqrt())
        );
    }

    #[test]
    fn test_angle_between() {
        let e1 = PatinaVec2f::new(1.0, 0.0);
        let e2 = PatinaVec2f::new(0.0, 1.0);

        assert_relative_eq!(e1.angle_between(&e2), PI / 2.0);
    }

    #[test]
    fn test_cross_product() {
        let e1 = PatinaVec2f::new(1.0, 0.0);
        let e2 = PatinaVec2f::new(0.0, 1.0);

        assert_eq!(e1.cross(&e2), 1.0);
    }
}
