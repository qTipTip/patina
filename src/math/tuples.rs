#![allow(dead_code)]

pub mod tuple2;
pub mod tuple3;

pub type TupleLength = f64;

// Phantom tags: Tuple2/Tuple3 carry one of these as a type parameter to say whether they are a
// point, a vector or a normal.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TypePoint {}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TypeVector {}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TypeNormal {}

// Tags whose tuples are directions. Dot products are defined between any two of them.
pub trait Direction {}
impl Direction for TypeVector {}
impl Direction for TypeNormal {}

// The tag of the result when subtracting two tuples with the same tag.
pub trait Difference {
    type Output;
}
impl Difference for TypePoint {
    type Output = TypeVector;
}
impl Difference for TypeVector {
    type Output = TypeVector;
}
impl Difference for TypeNormal {
    type Output = TypeNormal;
}
