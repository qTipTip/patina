#![allow(dead_code)]

use crate::math::{
    PatinaFloat, PatinaInt,
    vectors::{vec2::Vector2, vec3::Vector3},
};
pub mod vec2;
pub mod vec3;

pub type PatinaVec2f = Vector2<PatinaFloat>;
pub type PatinaVec2i = Vector2<PatinaInt>;
pub type PatinaVec3f<S> = Vector3<PatinaFloat, S>;
pub type PatinaVec3i<S> = Vector3<PatinaInt, S>;
