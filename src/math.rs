pub(crate) type PatinaFloat = f64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub(crate) struct Tuple2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub(crate) struct Tuple3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
