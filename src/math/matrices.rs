use std::ops::Index;

use crate::math::PatinaFloat;

pub struct SquareMatrix<const N: usize> {
    m: [[PatinaFloat; N]; N],
}

impl<const N: usize> Default for SquareMatrix<N> {
    fn default() -> Self {
        let mut m = [[0.0; N]; N];
        (0..N).for_each(|i| m[i][i] = 1.0);
        Self { m }
    }
}
impl<const N: usize> Index<usize> for SquareMatrix<N> {
    type Output = [PatinaFloat; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}
pub type Matrix4 = SquareMatrix<4>;

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::math::matrices::SquareMatrix;

    #[test]
    fn test_default() {
        let m = SquareMatrix::<10>::default();
        for i in 0..10 {
            for j in 0..10 {
                if i == j {
                    assert_relative_eq!(m[i][j], 1.0);
                } else {
                    assert_relative_eq!(m[i][j], 0.0);
                }
            }
        }
    }
}
