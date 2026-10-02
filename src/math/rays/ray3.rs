use crate::math::{PatinaFloat, points::PatinaPoint3f, vectors::PatinaVec3f};

#[derive(Copy, Clone)]
pub struct PatinaRay3f {
    o: PatinaPoint3f, // origin
    d: PatinaVec3f,   // direction
    _t: PatinaFloat,  // time
}

impl PatinaRay3f {
    pub fn new(o: PatinaPoint3f, d: PatinaVec3f) -> Self {
        Self { o, d, _t: 0.0 }
    }
    pub fn eval(self, t: PatinaFloat) -> PatinaPoint3f {
        debug_assert!(t >= 0.0);
        self.o + self.d * t
    }
}

#[cfg(test)]
mod test {
    use crate::math::{points::PatinaPoint3f, rays::ray3::PatinaRay3f, vectors::PatinaVec3f};

    #[test]
    fn test_ray_construction() {
        let o = PatinaPoint3f::zero();
        let d = PatinaVec3f::new(1.0, 0.5, 0.0);

        let r = PatinaRay3f::new(o, d);

        assert_eq!(r.eval(0.0), o);
        assert_eq!(r.eval(0.5), PatinaPoint3f::new(0.5, 0.25, 0.0));
        assert_eq!(r.eval(1.0), PatinaPoint3f::new(1.0, 0.5, 0.0));
    }
}
