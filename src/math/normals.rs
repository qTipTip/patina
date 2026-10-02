use crate::math::{
    PatinaFloat, PatinaInt,
    normals::{normal2::Normal2, normal3::Normal3},
};

pub mod normal2;
pub mod normal3;

pub type PatinaNorm3f = Normal3<PatinaFloat>;
pub type PatinaNorm3i = Normal3<PatinaInt>;
pub type PatinaNorm2f = Normal2<PatinaFloat>;
pub type PatinaNorm2i = Normal2<PatinaInt>;
