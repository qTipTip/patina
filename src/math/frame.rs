use crate::math::{PatinaNorm3f, PatinaVec3f, ShadingSpace};

/// An orthonormal basis (x, y, z) of directions in space `S`, used to move directions between `S`
/// and a local frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame<S> {
    x: PatinaVec3f<S>,
    y: PatinaVec3f<S>,
    z: PatinaVec3f<S>,
}

impl<S> Frame<S> {
    /// A frame whose z-axis is the given unit vector, with x and y chosen perpendicular to it.
    pub fn from_z(z: PatinaVec3f<S>) -> Self {
        let (z, x, y) = z.coordinate_system();
        Self { x, y, z }
    }

    /// A frame whose z-axis is the (unit) normal n.
    pub fn from_normal(n: PatinaNorm3f<S>) -> Self {
        Self::from_z(PatinaVec3f::new(n.x, n.y, n.z))
    }

    /// Expresses `v` in the local frame. The basis is orthonormal, so each local coordinate is the
    /// projection of `v` onto the corresponding axis.
    pub fn to_local(&self, v: PatinaVec3f<S>) -> PatinaVec3f<ShadingSpace> {
        PatinaVec3f::new(v.dot(&self.x), v.dot(&self.y), v.dot(&self.z))
    }

    /// The inverse of to_local: the local coordinates weigh the frame's axes.
    pub fn from_local(&self, v: PatinaVec3f<ShadingSpace>) -> PatinaVec3f<S> {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

// The frame's axes, for tests.
#[cfg(test)]
impl<S> Frame<S> {
    fn axes(&self) -> [PatinaVec3f<S>; 3] {
        [self.x, self.y, self.z]
    }
}

#[cfg(test)]
mod test_frame {
    use approx::assert_relative_eq;

    use crate::math::{Frame, PatinaVec3f, RenderSpace};

    fn tilted_frame() -> Frame<RenderSpace> {
        Frame::from_z(PatinaVec3f::new(1.0, 2.0, -0.5).normalize())
    }

    #[test]
    fn test_axes_are_orthonormal() {
        let axes = tilted_frame().axes();
        for (i, a) in axes.iter().enumerate() {
            assert_relative_eq!(a.len(), 1.0, epsilon = 1e-12);
            for b in &axes[i + 1..] {
                assert_relative_eq!(a.dot(b), 0.0, epsilon = 1e-12);
            }
        }
    }

    #[test]
    fn test_z_axis_maps_to_local_z() {
        let z = PatinaVec3f::<RenderSpace>::new(1.0, 2.0, -0.5).normalize();
        let frame = Frame::from_z(z);

        assert_relative_eq!(
            frame.to_local(z),
            PatinaVec3f::new(0.0, 0.0, 1.0),
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_round_trip() {
        let frame = tilted_frame();
        let v = PatinaVec3f::new(0.3, -1.2, 2.0);

        assert_relative_eq!(frame.from_local(frame.to_local(v)), v, epsilon = 1e-12);
    }
}
