mod tuples;
mod vectors;

pub(crate) type PatinaFloat = f64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}
