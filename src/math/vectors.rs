#![allow(dead_code)]

use crate::math::{
    PatinaFloat, PatinaInt,
    vectors::{vec2::Vector2, vec3::Vector3},
};
pub mod vec2;
pub mod vec3;

type PatinaVec2f = Vector2<PatinaFloat>;
type PatinaVec2i = Vector2<PatinaInt>;
type PatinaVec3f = Vector3<PatinaFloat>;
type PatinaVec3i = Vector3<PatinaInt>;
