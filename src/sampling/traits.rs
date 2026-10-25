use crate::math::{PatinaFloat, PatinaPoint2f};

pub trait Sampler {
    fn get_1d(&self) -> PatinaFloat;
    fn get_2d(&self) -> PatinaPoint2f;
    fn get_pixel_2d(&self) -> PatinaPoint2f;
}
