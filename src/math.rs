mod tuples;
mod vectors;

pub(crate) type PatinaFloat = f64;
pub(crate) type PatinaInt = i64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}
