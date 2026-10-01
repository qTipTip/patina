use crate::math::{
    traits::CheckNan,
    tuples::{TypeNormal, tuple2::Tuple2},
    vectors::vec2::Vector2,
};
use num_traits::Num;

pub type Normal2<T> = Tuple2<T, TypeNormal>;

impl<T: Num + Copy + CheckNan> From<Vector2<T>> for Normal2<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(test)]
mod normal_vector_interactions {
    use crate::math::{normals::normal2::Normal2, vectors::vec2::Vector2};

    #[test]
    fn dot_between_normals_and_vectors() {
        let n = Normal2::new(1.0, 2.0);
        let v = Vector2::new(4.0, 5.0);

        assert_eq!(n.dot(&v), 14.0);
        assert_eq!(v.dot(&n), 14.0);
        assert_eq!(n.dot(&n), 5.0);
        assert_eq!(v.dot(&v), 41.0);
    }

    #[test]
    fn normal_from_vector() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(Normal2::from(v), Normal2::new(1.0, 2.0));
    }
}
