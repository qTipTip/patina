pub(crate) mod bounding_boxes;
pub(crate) mod matrices;
pub(crate) mod normals;
pub(crate) mod points;
pub(crate) mod rays;
mod traits;
pub(crate) mod transforms;
pub(crate) mod tuples;
pub(crate) mod vectors;

// The public math API.
pub use bounding_boxes::{
    Bounds2f, Bounds2i, Bounds3f, Bounds3i, bounds2::Bounds2, bounds3::Bounds3,
};
pub use matrices::{Matrix4, SquareMatrix};
pub use normals::{
    PatinaNorm2f, PatinaNorm2i, PatinaNorm3f, PatinaNorm3i, normal2::Normal2, normal3::Normal3,
};
pub use points::{
    PatinaPoint2f, PatinaPoint2i, PatinaPoint3f, PatinaPoint3i, point2::Point2, point3::Point3,
};
pub use rays::{ray2::PatinaRay2f, ray3::PatinaRay3f};
pub use transforms::{Apply, Transform};
pub use tuples::{TupleLength, TypeNormal, TypePoint, TypeVector, tuple2::Tuple2, tuple3::Tuple3};
pub use vectors::{
    PatinaVec2f, PatinaVec2i, PatinaVec3f, PatinaVec3i, vec2::Vector2, vec3::Vector3,
};

pub type PatinaFloat = f64;
pub type PatinaInt = i64;

pub const MACHINE_EPSILON: PatinaFloat = PatinaFloat::EPSILON * 0.5;
pub const fn gamma(n: i32) -> PatinaFloat {
    let n = n as PatinaFloat;
    (n * MACHINE_EPSILON) / (1.0 - n * MACHINE_EPSILON)
}

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}

pub(crate) fn safe_asin(x: PatinaFloat) -> PatinaFloat {
    x.clamp(-1.0, 1.0).asin()
}

#[allow(dead_code)]
pub(crate) fn safe_acos(x: PatinaFloat) -> PatinaFloat {
    x.clamp(-1.0, 1.0).acos()
}
