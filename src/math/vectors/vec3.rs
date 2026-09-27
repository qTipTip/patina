use crate::math::{PatinaFloat, PatinaInt, tuples::tuple3::Tuple3};

struct Vector3<T>(Tuple3<T>);

type PatinaVec3f = Vector3<PatinaFloat>;
type PatinaVec3i = Vector3<PatinaInt>;
