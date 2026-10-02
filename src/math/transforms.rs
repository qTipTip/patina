use std::ops::Mul;

use approx::relative_ne;

use crate::math::{
    PatinaFloat, matrices::Matrix4, normals::PatinaNorm3f, points::PatinaPoint3f,
    rays::ray3::PatinaRay3f, vectors::PatinaVec3f,
};

// A transform encodes a transformation matrix (4x4) along with it's inverse (None if singular).
#[derive(Debug, PartialEq)]
pub struct Transform {
    m: Matrix4,
    m_inv: Matrix4,
}

pub trait Apply<In> {
    type Output;
    fn apply(&self, x: In) -> Self::Output;
}

impl Transform {
    // Return an Option<transform> defined by the matrix `m`. Is None if the matrix m cannot be
    // inverted.
    pub fn from_matrix(m: &Matrix4) -> Option<Self> {
        if let Some(m_inv) = m.inverse() {
            return Some(Self { m: *m, m_inv });
        }
        None
    }

    pub fn translate(u: &PatinaVec3f) -> Self {
        let mut m = Matrix4::identity();
        let mut m_inv = Matrix4::identity();

        m[0][3] = u.x;
        m[1][3] = u.y;
        m[2][3] = u.z;

        m_inv[0][3] = -u.x;
        m_inv[1][3] = -u.y;
        m_inv[2][3] = -u.z;

        Self { m, m_inv }
    }

    pub fn scale(x: PatinaFloat, y: PatinaFloat, z: PatinaFloat) -> Self {
        let m = Matrix4::diag([x, y, z, 1.0]);
        let m_inv = Matrix4::diag([1.0 / x, 1.0 / y, 1.0 / z, 1.0]);

        Self { m, m_inv }
    }

    // method for checking if a particular transformation has scaling terms or not.
    pub fn has_scaling(&self) -> bool {
        let e1 = PatinaVec3f::new(1.0, 0.0, 0.0);
        let e2 = PatinaVec3f::new(0.0, 1.0, 0.0);
        let e3 = PatinaVec3f::new(0.0, 0.0, 1.0);

        // If any of the unit vectors have changed length, then we return true
        relative_ne!(self.apply(e1).len_squared(), 1.0)
            || relative_ne!(self.apply(e2).len_squared(), 1.0)
            || relative_ne!(self.apply(e3).len_squared(), 1.0)
    }

    // for a left handed coordinate system, this is a clockwise rotation around x-axis.
    pub fn rotate_x(&self, theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([1.0, cos_theta, cos_theta, 1.0]);
        m[2][1] = sin_theta;
        m[1][2] = -sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self {
            m,
            m_inv: m.transpose(),
        }
    }

    // for a left handed coordinate system, this is a clockwise rotation around y-axis.
    pub fn rotate_y(&self, theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([cos_theta, 1.0, cos_theta, 1.0]);
        m[2][0] = -sin_theta;
        m[0][2] = sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self {
            m,
            m_inv: m.transpose(),
        }
    }

    // for a left handed coordinate system, this is a clockwise rotation around z-axis.
    pub fn rotate_z(&self, theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([cos_theta, cos_theta, 1.0, 1.0]);
        m[0][1] = -sin_theta;
        m[1][0] = -sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self {
            m,
            m_inv: m.transpose(),
        }
    }

    // Returns the inverse transform
    pub fn inverse(&self) -> Self {
        Self {
            m: self.m_inv,
            m_inv: self.m,
        }
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m * rhs.m,
            m_inv: rhs.m_inv * self.m_inv,
        }
    }
}

impl Apply<PatinaPoint3f> for Transform {
    type Output = PatinaPoint3f;

    fn apply(&self, p: PatinaPoint3f) -> Self::Output {
        let v = [p.x, p.y, p.z, 1.0];
        let q = self.m * v;

        let w = q[3];
        if w == 1.0 {
            PatinaPoint3f::new(q[0], q[1], q[2])
        } else {
            PatinaPoint3f::new(q[0] / w, q[1] / w, q[2] / w)
        }
    }
}

impl Apply<PatinaVec3f> for Transform {
    type Output = PatinaVec3f;

    fn apply(&self, v: PatinaVec3f) -> Self::Output {
        let v = [v.x, v.y, v.z, 0.0];
        let q = self.m * v;

        PatinaVec3f::new(q[0], q[1], q[2])
    }
}

impl Apply<PatinaNorm3f> for Transform {
    type Output = PatinaNorm3f;
    // Normals do not transform like vectors. They instead transform by the transposed inverse.
    fn apply(&self, n: PatinaNorm3f) -> Self::Output {
        let v = [n.x, n.y, n.z, 0.0];
        let inv_t = self.m_inv.transpose();

        let w = inv_t * v;
        PatinaNorm3f::new(w[0], w[1], w[2])
    }
}

impl Apply<PatinaRay3f> for Transform {
    type Output = PatinaRay3f;

    fn apply(&self, r: PatinaRay3f) -> Self::Output {
        // Transform the origin and direction separately
        PatinaRay3f::new(self.apply(r.o), self.apply(r.d))
    }
}

#[cfg(test)]
mod test_transforms {
    use crate::math::{
        normals::normal3::Normal3,
        points::PatinaPoint3f,
        transforms::{Apply, Transform},
        vectors::PatinaVec3f,
    };

    #[test]
    fn test_inverse_is_identity() {
        let t = Transform::scale(2.0, 1.0, 0.5);
        let p = PatinaPoint3f::new(1.0, 0.5, 0.25);
        // inverting the inverse of a transform is the transform
        assert_eq!(t.inverse().inverse(), t);
        // applying t, then t inverse is the identity
        assert_eq!((t.inverse() * t).apply(p), p);
    }

    #[test]
    fn test_translation() {
        let t = Transform::translate(&PatinaVec3f::new(2.0, 1.0, -0.5));
        let p = PatinaPoint3f::new(0.1, 0.2, 0.3);
        let v = PatinaVec3f::new(0.1, 0.2, 0.3);

        // translations move points,
        assert_eq!(t.apply(p), PatinaPoint3f::new(2.1, 1.2, -0.2));
        // but not vectors
        assert_eq!(t.apply(v), v);
    }

    #[test]
    fn test_normals_scale() {
        let s = Transform::scale(1.2, 2.1, -1.2);
        assert!(s.has_scaling());

        let (e1, e2, _) = PatinaVec3f::new(15.0, 1.0, 2.0)
            .normalize()
            .coordinate_system();
        let n = Normal3::from(e1);

        // n and e2 should be perpendicular
        assert_eq!(n.dot(&e2), 0.0);
        // S*n and S*e2 should still be perpendicular after a
        assert_eq!(s.apply(n).dot(&s.apply(e2)), 0.0);
    }
}
