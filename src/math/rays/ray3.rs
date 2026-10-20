use approx::{AbsDiffEq, RelativeEq};

use crate::math::{PatinaFloat, points::PatinaPoint3f, vectors::PatinaVec3f};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PatinaRay3f<S> {
    pub o: PatinaPoint3f<S>, // origin
    pub d: PatinaVec3f<S>,   // direction
    pub t: PatinaFloat,      // time
}

impl<S: PartialEq> AbsDiffEq for PatinaRay3f<S> {
    type Epsilon = PatinaFloat;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.o.abs_diff_eq(&other.o, epsilon)
            && self.d.abs_diff_eq(&other.d, epsilon)
            && self.t.abs_diff_eq(&other.t, epsilon)
    }
}

impl<S: PartialEq> RelativeEq for PatinaRay3f<S> {
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
            && self.t.relative_eq(&other.t, epsilon, max_relative)
    }
}

impl<S> PatinaRay3f<S> {
    pub fn new(o: PatinaPoint3f<S>, d: PatinaVec3f<S>, t: PatinaFloat) -> Self {
        Self { o, d, t }
    }
    pub fn eval(self, t: PatinaFloat) -> PatinaPoint3f<S> {
        debug_assert!(t >= 0.0);
        self.o + self.d * t
    }
}

#[cfg(test)]
mod test {
    use crate::math::WorldSpace;
    use crate::math::{
        PatinaFloat, points::PatinaPoint3f, rays::ray3::PatinaRay3f, vectors::PatinaVec3f,
    };

    #[test]
    fn test_ray_construction() {
        let o = PatinaPoint3f::<WorldSpace>::zero();
        let d = PatinaVec3f::<WorldSpace>::new(1.0, 0.5, 0.0);
        let t: PatinaFloat = 0.0;

        let r = PatinaRay3f::<WorldSpace>::new(o, d, t);

        assert_eq!(r.eval(0.0), o);
        assert_eq!(
            r.eval(0.5),
            PatinaPoint3f::<WorldSpace>::new(0.5, 0.25, 0.0)
        );
        assert_eq!(r.eval(1.0), PatinaPoint3f::<WorldSpace>::new(1.0, 0.5, 0.0));
    }
}

#[cfg(test)]
mod test_approx {
    use crate::math::WorldSpace;
    use approx::{assert_relative_eq, relative_ne};

    use crate::math::{points::PatinaPoint3f, rays::ray3::PatinaRay3f, vectors::PatinaVec3f};

    #[test]
    fn test_rays_compare_every_field() {
        let o = PatinaPoint3f::<WorldSpace>::new(0.1 + 0.2, 0.0, 0.0);
        let d = PatinaVec3f::<WorldSpace>::new(0.0, 0.0, 1.0);
        let r = PatinaRay3f::<WorldSpace>::new(o, d, 0.5);

        assert_relative_eq!(
            r,
            PatinaRay3f::<WorldSpace>::new(PatinaPoint3f::<WorldSpace>::new(0.3, 0.0, 0.0), d, 0.5)
        );
        assert!(relative_ne!(r, PatinaRay3f::<WorldSpace>::new(o, d, 0.6)));
        assert!(relative_ne!(r, PatinaRay3f::<WorldSpace>::new(o, -d, 0.5)));
    }
}
