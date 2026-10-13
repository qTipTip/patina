use approx::{AbsDiffEq, RelativeEq};

use crate::math::{PatinaFloat, points::PatinaPoint2f, vectors::PatinaVec2f};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PatinaRay2f {
    o: PatinaPoint2f, // origin
    d: PatinaVec2f,   // direction
    _t: PatinaFloat,  // time
}

impl AbsDiffEq for PatinaRay2f {
    type Epsilon = PatinaFloat;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.o.abs_diff_eq(&other.o, epsilon)
            && self.d.abs_diff_eq(&other.d, epsilon)
            && self._t.abs_diff_eq(&other._t, epsilon)
    }
}

impl RelativeEq for PatinaRay2f {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-12
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.o.relative_eq(&other.o, epsilon, max_relative)
            && self.d.relative_eq(&other.d, epsilon, max_relative)
            && self._t.relative_eq(&other._t, epsilon, max_relative)
    }
}

impl PatinaRay2f {
    pub fn new(o: PatinaPoint2f, d: PatinaVec2f) -> Self {
        Self { o, d, _t: 0.0 }
    }
    pub fn eval(self, t: PatinaFloat) -> PatinaPoint2f {
        debug_assert!(t >= 0.0);
        self.o + self.d * t
    }
}

#[cfg(test)]
mod test {
    use crate::math::{points::PatinaPoint2f, rays::ray2::PatinaRay2f, vectors::PatinaVec2f};

    #[test]
    fn test_ray_construction() {
        let o = PatinaPoint2f::zero();
        let d = PatinaVec2f::new(1.0, 0.5);

        let r = PatinaRay2f::new(o, d);

        assert_eq!(r.eval(0.0), o);
        assert_eq!(r.eval(0.5), PatinaPoint2f::new(0.5, 0.25));
        assert_eq!(r.eval(1.0), PatinaPoint2f::new(1.0, 0.5));
    }
}
