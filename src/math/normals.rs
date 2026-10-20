use crate::math::{
    PatinaFloat, PatinaInt,
    normals::{normal2::Normal2, normal3::Normal3},
};

pub mod normal2;
pub mod normal3;

pub type PatinaNorm3f<S> = Normal3<PatinaFloat, S>;
pub type PatinaNorm3i<S> = Normal3<PatinaInt, S>;
pub type PatinaNorm2f = Normal2<PatinaFloat>;
pub type PatinaNorm2i = Normal2<PatinaInt>;
