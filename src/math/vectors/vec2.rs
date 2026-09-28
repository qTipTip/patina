use crate::{
    implement_geometry_ops,
    math::traits::CheckNan,
    math::{PatinaFloat, PatinaInt, tuples::tuple2::Tuple2},
};
use num_traits::Num;

struct Vector2<T>(Tuple2<T>);

type PatinaVec2f = Vector2<PatinaFloat>;
type PatinaVec2i = Vector2<PatinaInt>;

implement_geometry_ops!(Vector2, Tuple2);
