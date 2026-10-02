use crate::math::{PatinaFloat, points::PatinaPoint2f, vectors::PatinaVec2f};

#[derive(Copy, Clone)]
pub struct PatinaRay2f {
    o: PatinaPoint2f, // origin
    d: PatinaVec2f,   // direction
    _t: PatinaFloat,  // time
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
