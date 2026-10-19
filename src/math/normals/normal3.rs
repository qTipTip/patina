use crate::math::{
    traits::CheckNan,
    tuples::{TypeNormal, tuple3::Tuple3},
    vectors::vec3::Vector3,
};
use num_traits::Num;

pub type Normal3<T, S> = Tuple3<T, TypeNormal, S>;

impl<T: Num + Copy + CheckNan, S> From<Vector3<T, S>> for Normal3<T, S> {
    fn from(value: Vector3<T, S>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(test)]
mod normal_vector_interactions {
    use crate::math::WorldSpace;
    use crate::math::{normals::normal3::Normal3, vectors::vec3::Vector3};

    #[test]
    fn dot_between_normals_and_vectors() {
        let n = Normal3::<_, WorldSpace>::new(1.0, 2.0, 3.0);
        let v = Vector3::<_, WorldSpace>::new(4.0, 5.0, 6.0);

        assert_eq!(n.dot(&v), 32.0);
        assert_eq!(v.dot(&n), 32.0);
        assert_eq!(n.dot(&n), 14.0);
        assert_eq!(v.dot(&v), 77.0);
    }

    #[test]
    fn normal_from_vector() {
        let v = Vector3::<_, WorldSpace>::new(1.0, 2.0, 3.0);
        assert_eq!(
            Normal3::from(v),
            Normal3::<_, WorldSpace>::new(1.0, 2.0, 3.0)
        );
    }
}
