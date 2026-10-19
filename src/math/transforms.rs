use std::{fmt, marker::PhantomData, ops::Mul};

use approx::{AbsDiffEq, RelativeEq, relative_ne};

use crate::math::{
    Bounds3f, PatinaFloat, matrices::Matrix4, normals::PatinaNorm3f, points::PatinaPoint3f,
    rays::ray3::PatinaRay3f, vectors::PatinaVec3f,
};

/// A transform from the space `From` to the space `To`, with its 4x4 matrix and the matrix's
/// inverse. The spaces are type-level tags (see `math::spaces`), named in the same order as the
/// a_from_b convention: `render_from_object` is a `Transform<RenderSpace, ObjectSpace>`.
///
/// Transforms only compose when the spaces in between match:
///
/// ```
/// use patina::math::{CameraSpace, RenderSpace, Transform, WorldSpace};
///
/// let world_from_camera: Transform<WorldSpace, CameraSpace> = Transform::identity();
/// let render_from_world: Transform<RenderSpace, WorldSpace> = Transform::identity();
/// let render_from_camera: Transform<RenderSpace, CameraSpace> = render_from_world * world_from_camera;
/// ```
///
/// Composing them in the wrong order doesn't compile:
///
/// ```compile_fail
/// use patina::math::{CameraSpace, RenderSpace, Transform, WorldSpace};
///
/// let world_from_camera: Transform<WorldSpace, CameraSpace> = Transform::identity();
/// let render_from_world: Transform<RenderSpace, WorldSpace> = Transform::identity();
/// let wrong = world_from_camera * render_from_world;
/// ```
pub struct Transform<To, From> {
    m: Matrix4,
    m_inv: Matrix4,
    _spaces: PhantomData<fn(From) -> To>,
}

// Implemented by hand rather than derived, as deriving would require the space tags to implement
// these traits too.
impl<To, From> Clone for Transform<To, From> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<To, From> Copy for Transform<To, From> {}

impl<To, From> PartialEq for Transform<To, From> {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m && self.m_inv == other.m_inv
    }
}

impl<To, From> fmt::Debug for Transform<To, From> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transform")
            .field("m", &self.m)
            .field("m_inv", &self.m_inv)
            .finish()
    }
}

impl<To, From> AbsDiffEq for Transform<To, From> {
    type Epsilon = PatinaFloat;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.m.abs_diff_eq(&other.m, epsilon) && self.m_inv.abs_diff_eq(&other.m_inv, epsilon)
    }
}

impl<To, From> RelativeEq for Transform<To, From> {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-12
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.m.relative_eq(&other.m, epsilon, max_relative)
            && self.m_inv.relative_eq(&other.m_inv, epsilon, max_relative)
    }
}

pub trait Apply<In> {
    type Output;
    fn apply(&self, x: In) -> Self::Output;
}

impl<To, From> Transform<To, From> {
    // The spaces are fixed by the type, so every constructor goes through here.
    fn from_parts(m: Matrix4, m_inv: Matrix4) -> Self {
        Self {
            m,
            m_inv,
            _spaces: PhantomData,
        }
    }

    // Return an Option<transform> defined by the matrix `m`. Is None if the matrix m cannot be
    // inverted.
    pub fn from_matrix(m: &Matrix4) -> Option<Self> {
        m.inverse().map(|m_inv| Self::from_parts(*m, m_inv))
    }

    pub fn identity() -> Self {
        Self::from_parts(Matrix4::identity(), Matrix4::identity())
    }

    // Reinterprets the transform as one between other spaces, without changing it. Only for
    // places where spaces coincide by construction, such as render space being camera space with
    // RenderingSpace::Camera.
    pub(crate) fn assume_spaces<NewTo, NewFrom>(self) -> Transform<NewTo, NewFrom> {
        Transform::from_parts(self.m, self.m_inv)
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

        Self::from_parts(m, m_inv)
    }

    pub fn scale(x: PatinaFloat, y: PatinaFloat, z: PatinaFloat) -> Self {
        let m = Matrix4::diag([x, y, z, 1.0]);
        let m_inv = Matrix4::diag([1.0 / x, 1.0 / y, 1.0 / z, 1.0]);

        Self::from_parts(m, m_inv)
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
    pub fn rotate_x(theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([1.0, cos_theta, cos_theta, 1.0]);
        m[2][1] = sin_theta;
        m[1][2] = -sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self::from_parts(m, m.transpose())
    }

    // for a left handed coordinate system, this is a clockwise rotation around y-axis.
    pub fn rotate_y(theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([cos_theta, 1.0, cos_theta, 1.0]);
        m[2][0] = -sin_theta;
        m[0][2] = sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self::from_parts(m, m.transpose())
    }

    // for a left handed coordinate system, this is a clockwise rotation around z-axis.
    pub fn rotate_z(theta: PatinaFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mut m = Matrix4::diag([cos_theta, cos_theta, 1.0, 1.0]);
        m[0][1] = -sin_theta;
        m[1][0] = sin_theta;

        // a rotation matrix is orthogonal, so it's inverse is the transpose
        Self::from_parts(m, m.transpose())
    }

    // Returns the inverse transform, which goes the other way between the two spaces.
    pub fn inverse(&self) -> Transform<From, To> {
        Transform::from_parts(self.m_inv, self.m)
    }

    // Computes the orthographic transformation which leaves (x, y) but transforms z-values at
    // z_near to 0 and z_values to z_far to 1.
    pub fn orthographic(z_near: PatinaFloat, z_far: PatinaFloat) -> Self {
        debug_assert!(z_far > z_near);
        // Translate within the source space, then scale into the target space.
        Self::scale(1.0, 1.0, 1.0 / (z_far - z_near))
            * Transform::<From, From>::translate(&PatinaVec3f::new(0.0, 0.0, -z_near))
    }

    // Computes the perspective projection. Takes a field of view andle and distances to the z_near,
    // z_far planes.
    pub fn perspective(fov: PatinaFloat, z_near: PatinaFloat, z_far: PatinaFloat) -> Self {
        debug_assert!(
            z_near > 0.0,
            "z_near must be strictly positive, otherwise the transform has no inverse"
        );
        debug_assert!(z_far > z_near);
        // Compute the transform for perspecitve projection
        let mut m = Matrix4::identity();
        m[2][2] = z_far / (z_far - z_near);
        m[2][3] = -z_far * z_near / (z_far - z_near);
        m[3][2] = 1.0;
        m[3][3] = 0.0;

        let t = Transform::<From, From>::from_matrix(&m).expect("perspective matrix is invertible");
        // Field of view compensation
        let inv_tan_angle = 1.0 / (fov * 0.5).tan();

        Self::scale(inv_tan_angle, inv_tan_angle, 1.0) * t
    }
}

// a_from_b * b_from_c = a_from_c: the spaces in between must match.
impl<A, B, C> Mul<Transform<B, C>> for Transform<A, B> {
    type Output = Transform<A, C>;

    fn mul(self, rhs: Transform<B, C>) -> Self::Output {
        Transform::from_parts(self.m * rhs.m, rhs.m_inv * self.m_inv)
    }
}

impl<To, From> Apply<PatinaPoint3f> for Transform<To, From> {
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

impl<To, From> Apply<PatinaVec3f> for Transform<To, From> {
    type Output = PatinaVec3f;

    fn apply(&self, v: PatinaVec3f) -> Self::Output {
        let v = [v.x, v.y, v.z, 0.0];
        let q = self.m * v;

        PatinaVec3f::new(q[0], q[1], q[2])
    }
}

impl<To, From> Apply<PatinaNorm3f> for Transform<To, From> {
    type Output = PatinaNorm3f;
    // Normals do not transform like vectors. They instead transform by the transposed inverse.
    fn apply(&self, n: PatinaNorm3f) -> Self::Output {
        let v = [n.x, n.y, n.z, 0.0];
        let inv_t = self.m_inv.transpose();

        let w = inv_t * v;
        PatinaNorm3f::new(w[0], w[1], w[2])
    }
}

impl<To, From> Apply<PatinaRay3f> for Transform<To, From> {
    type Output = PatinaRay3f;

    fn apply(&self, r: PatinaRay3f) -> Self::Output {
        // Transform the origin and direction separately
        PatinaRay3f::new(self.apply(r.o), self.apply(r.d), r.t)
    }
}

impl<To, From> Apply<Bounds3f> for Transform<To, From> {
    type Output = Bounds3f;

    fn apply(&self, x: Bounds3f) -> Self::Output {
        Bounds3f::new(self.apply(x.p_min), self.apply(x.p_max))
    }
}

#[cfg(test)]
mod test_transforms {
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use crate::math::{
        normals::normal3::Normal3,
        points::PatinaPoint3f,
        spaces::WorldSpace,
        transforms::{Apply, Transform},
        vectors::PatinaVec3f,
    };

    // These tests are about the math, not the spaces, so every transform maps world space to itself.
    type T = Transform<WorldSpace, WorldSpace>;

    #[test]
    fn test_inverse_is_identity() {
        let t = T::scale(2.0, 1.0, 0.5);
        let p = PatinaPoint3f::new(1.0, 0.5, 0.25);
        // inverting the inverse of a transform is the transform
        assert_abs_diff_eq!(t.inverse().inverse(), t);
        // applying t, then t inverse is the identity
        assert_eq!((t.inverse() * t).apply(p), p);
    }

    #[test]
    fn test_translation() {
        let t = T::translate(&PatinaVec3f::new(2.0, 1.0, -0.5));
        let p = PatinaPoint3f::new(0.1, 0.2, 0.3);
        let v = PatinaVec3f::new(0.1, 0.2, 0.3);

        // translations move points,
        assert_eq!(t.apply(p), PatinaPoint3f::new(2.1, 1.2, -0.2));
        // but not vectors
        assert_eq!(t.apply(v), v);
    }

    #[test]
    fn test_normals_scale() {
        let s = T::scale(1.2, 2.1, -1.2);
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

    #[test]
    fn test_rotation_around_x_is_identity() {
        let mut m = T::identity();
        let s = T::rotate_x(2.0 * PI / 4.0);
        for _ in 0..4 {
            m = m * s;
        }
        assert_abs_diff_eq!(m, T::identity())
    }
    #[test]
    fn test_rotation_around_y_is_identity() {
        let mut m = T::identity();
        let s = T::rotate_y(2.0 * PI / 4.0);
        for _ in 0..4 {
            m = m * s;
        }
        assert_abs_diff_eq!(m, T::identity())
    }
    #[test]
    fn test_rotation_around_z_is_identity() {
        let mut m = T::identity();
        let s = T::rotate_z(2.0 * PI / 4.0);
        for _ in 0..4 {
            m = m * s;
        }
        assert_abs_diff_eq!(m, T::identity())
    }
}
