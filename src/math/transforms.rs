use crate::math::matrices::Matrix4;

// A transform encodes a transformation matrix (4x4) along with it's inverse (None if singular).
pub struct Transform {
    m: Matrix4,
    m_inv: Option<Matrix4>,
}

impl Transform {
    // Return an Option<transform> defined by the matrix `m`. Is None if the matrix m cannot be
    // inverted.
    pub fn from_matrix(m: &Matrix4) -> Option<Self> {
        Self {
            m,
            m_inv: m.inverse(),
        }
    }
}
