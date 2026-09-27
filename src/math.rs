pub(crate) type PatinaFloat = f64;

#[derive(Clone, Copy)]
pub(crate) struct Point2D {
    pub x: PatinaFloat,
    pub y: PatinaFloat,
}

#[derive(Clone, Copy)]
pub(crate) struct Tuple2 {
    pub x: PatinaFloat,
    pub y: PatinaFloat,
}

#[derive(Clone, Copy)]
pub(crate) struct Tuple3 {
    pub x: PatinaFloat,
    pub y: PatinaFloat,
    pub z: PatinaFloat,
}
