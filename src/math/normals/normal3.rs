use crate::math::{
    traits::CheckNan,
    tuples::{TypeNormal, tuple3::Tuple3},
    vectors::vec3::Vector3,
};
use num_traits::Num;

pub type Normal3<T> = Tuple3<T, TypeNormal>;

impl<T: Num + Copy + CheckNan> From<Vector3<T>> for Normal3<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(test)]
mod normal_vector_interactions {
    use crate::math::{normals::normal3::Normal3, vectors::vec3::Vector3};

    #[test]
    fn dot_between_normals_and_vectors() {
        let n = Normal3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(4.0, 5.0, 6.0);

        assert_eq!(n.dot(&v), 32.0);
        assert_eq!(v.dot(&n), 32.0);
        assert_eq!(n.dot(&n), 14.0);
        assert_eq!(v.dot(&v), 77.0);
    }

    #[test]
    fn normal_from_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(Normal3::from(v), Normal3::new(1.0, 2.0, 3.0));
    }
}
