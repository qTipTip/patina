pub mod normals;
pub mod points;
pub mod rays;
mod traits;
pub mod tuples;
pub mod vectors;

pub type PatinaFloat = f64;
pub type PatinaInt = i64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}

pub fn safe_asin(x: PatinaFloat) -> PatinaFloat {
    x.clamp(-1.0, 1.0).asin()
}

pub fn safe_acos(x: PatinaFloat) -> PatinaFloat {
    x.clamp(-1.0, 1.0).acos()
}
