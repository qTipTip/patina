use crate::math::{
    PatinaFloat, PatinaInt,
    bounding_boxes::{bounds2::Bounds2, bounds3::Bounds3},
};

pub mod bounds2;
pub mod bounds3;

pub type Bounds2f = Bounds2<PatinaFloat>;
pub type Bounds2i = Bounds2<PatinaInt>;
pub type Bounds3f<S> = Bounds3<PatinaFloat, S>;
pub type Bounds3i<S> = Bounds3<PatinaInt, S>;
