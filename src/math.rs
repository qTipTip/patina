pub mod tuples;
pub mod vectors;

pub type PatinaFloat = f64;
pub type PatinaInt = i64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}
