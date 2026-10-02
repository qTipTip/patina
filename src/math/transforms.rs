use crate::math::matrices::Matrix4;

// A transform encodes a transformation matrix (4x4) along with it's inverse (None if singular).
pub struct Transform {
    m: Matrix4,
    m_inv: Option<Matrix4>,
}
