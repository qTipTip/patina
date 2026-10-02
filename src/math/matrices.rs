#[derive(Default)]
pub struct SquareMatrix<const N: usize> {
    m: [[PatinaFloat; N]; N],
}

pub type Matrix4 = SquareMatrix<4>;
