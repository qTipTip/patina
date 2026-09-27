use crate::math::{PatinaFloat, PatinaInt, tuples::tuple2::Tuple2};

struct Vector2<T>(Tuple2<T>);

type PatinaVec2f = Vector2<PatinaFloat>;
type PatinaVec2i = Vector2<PatinaInt>;
