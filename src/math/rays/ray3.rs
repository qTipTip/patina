use crate::math::{PatinaFloat, points::PatinaPoint3f, vectors::PatinaVec3f};

pub struct PatinaRay3 {
    o: PatinaPoint3f,
    d: PatinaVec3f,
    t: PatinaFloat,
}

impl PatinaRay3 {
    pub fn new(self, o: PatinaPoint3f, d: PatinaVec3f) -> Self {
        Self { o, d, t: 0.0 }
    }
    pub fn eval(self, t: PatinaFloat) -> PatinaPoint3f {
        debug_assert!(t >= 0.0);
        self.o + self.d * t
    }
}
