use num_traits::Float;

#[derive(Clone, Copy)]
pub(crate) struct Tuple2<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
}
