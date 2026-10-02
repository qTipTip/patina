use std::ops::{Add, Index, IndexMut, Mul};

use crate::math::PatinaFloat;

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl<const N: usize> IndexMut<usize> for SquareMatrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

pub type Matrix4 = SquareMatrix<4>;

impl<const N: usize> SquareMatrix<N> {
    // Rows are given in order, so `m[i][j]` is row i, column j.
    pub fn new(m: [[PatinaFloat; N]; N]) -> Self {
        Self { m }
    }

    pub fn identity() -> Self {
        Self::default()
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; N]; N] }
    }

    pub fn diag(d: [PatinaFloat; N]) -> Self {
        let mut m = [[0.0; N]; N];
        (0..N).for_each(|i| m[i][i] = d[i]);
        Self { m }
    }

    pub fn is_identity(&self) -> bool {
        *self == Self::identity()
    }

    pub fn transpose(&self) -> Self {
        let mut m = [[0.0; N]; N];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = self.m[j][i];
            }
        }
        Self { m }
    }

    // Gaussian elimination with partial pivoting.
    pub fn determinant(&self) -> PatinaFloat {
        let mut a = self.m;
        let mut det = 1.0;
        for col in 0..N {
            let pivot = Self::pivot_row(&a, col);
            if a[pivot][col] == 0.0 {
                return 0.0;
            }
            if pivot != col {
                a.swap(pivot, col);
                det = -det;
            }
            det *= a[col][col];
            let pivot_values = a[col];
            for row in &mut a[col + 1..] {
                let f = row[col] / pivot_values[col];
                for (value, p) in row[col..].iter_mut().zip(&pivot_values[col..]) {
                    *value -= f * p;
                }
            }
        }
        det
    }

    // Using a naive inverse for now (Gauss-Jordan elimination with partial pivoting). But, future
    // optimization might be possible if we assume that we're dealing with 4x4 transformation
    // matrices:
    // https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html
    pub fn inverse(&self) -> Option<Self> {
        let mut a = self.m;
        let mut inv = Self::identity().m;
        for col in 0..N {
            let pivot = Self::pivot_row(&a, col);
            if a[pivot][col] == 0.0 {
                return None;
            }
            a.swap(pivot, col);
            inv.swap(pivot, col);

            let p = a[col][col];
            for k in 0..N {
                a[col][k] /= p;
                inv[col][k] /= p;
            }
            for row in 0..N {
                if row == col {
                    continue;
                }
                let f = a[row][col];
                for k in 0..N {
                    a[row][k] -= f * a[col][k];
                    inv[row][k] -= f * inv[col][k];
                }
            }
        }
        Some(Self { m: inv })
    }

    // The row at or below `col` with the largest absolute value in column `col`.
    fn pivot_row(a: &[[PatinaFloat; N]; N], col: usize) -> usize {
        (col..N)
            .max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs()))
            .expect("col must be less than N")
    }
}

impl<const N: usize> Mul for SquareMatrix<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = (0..N).map(|k| self.m[i][k] * rhs.m[k][j]).sum();
            }
        }
        Self { m }
    }
}

impl<const N: usize> Mul<PatinaFloat> for SquareMatrix<N> {
    type Output = Self;
    fn mul(self, rhs: PatinaFloat) -> Self::Output {
        Self {
            m: self.m.map(|row| row.map(|value| value * rhs)),
        }
    }
}

impl<const N: usize> Add for SquareMatrix<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut m = self.m;
        for (row, rhs_row) in m.iter_mut().zip(rhs.m) {
            for (value, rhs_value) in row.iter_mut().zip(rhs_row) {
                *value += rhs_value;
            }
        }
        Self { m }
    }
}

// An algebraic matrix x vector multiplication. It does not carry any geometric meaning.
impl<const N: usize> Mul<[PatinaFloat; N]> for SquareMatrix<N> {
    type Output = [PatinaFloat; N];

    fn mul(self, rhs: [PatinaFloat; N]) -> Self::Output {
        std::array::from_fn(|i| self[i].iter().zip(rhs).map(|(a, b)| a * b).sum())
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::math::matrices::{Matrix4, SquareMatrix};

    fn assert_matrix_eq(a: &Matrix4, b: &Matrix4) {
        for i in 0..4 {
            for j in 0..4 {
                assert_relative_eq!(a[i][j], b[i][j], epsilon = 1e-12);
            }
        }
    }

    // An invertible matrix (determinant -28) that needs row swaps during elimination.
    fn sample() -> Matrix4 {
        Matrix4::new([
            [2.0, 0.0, 1.0, 3.0],
            [1.0, 1.0, 0.0, 2.0],
            [0.0, 4.0, 1.0, 1.0],
            [3.0, 1.0, 2.0, 0.0],
        ])
    }

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

    #[test]
    fn test_identity_and_diag() {
        assert!(Matrix4::identity().is_identity());
        assert!(Matrix4::diag([1.0; 4]).is_identity());
        assert!(!Matrix4::diag([1.0, 2.0, 1.0, 1.0]).is_identity());
        assert!(!Matrix4::zero().is_identity());
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix4::zero();
        m[1][2] = 5.0;
        assert_eq!(m[1][2], 5.0);
    }

    #[test]
    fn test_transpose() {
        let m = sample();
        let t = m.transpose();
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(t[i][j], m[j][i]);
            }
        }
        assert_eq!(t.transpose(), m);
    }

    #[test]
    fn test_mul_identity() {
        let m = sample();
        assert_eq!(m * Matrix4::identity(), m);
        assert_eq!(Matrix4::identity() * m, m);
    }

    #[test]
    fn test_mul() {
        let a = Matrix4::new([
            [1.0, 2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Matrix4::diag([2.0, 3.0, 4.0, 5.0]);
        let expected = Matrix4::new([
            [2.0, 6.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 5.0],
        ]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_scalar_mul_and_add() {
        let m = sample();
        assert_eq!(m * 2.0, m + m);
        assert_eq!(m + Matrix4::zero(), m);
    }

    #[test]
    fn test_determinant() {
        assert_relative_eq!(Matrix4::identity().determinant(), 1.0);
        assert_relative_eq!(Matrix4::diag([2.0, 3.0, 4.0, 5.0]).determinant(), 120.0);
        assert_relative_eq!(sample().determinant(), -28.0, epsilon = 1e-12);
        assert_relative_eq!(
            sample().transpose().determinant(),
            sample().determinant(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_inverse() {
        let m = sample();
        let inv = m.inverse().expect("sample matrix is invertible");
        assert_matrix_eq(&(m * inv), &Matrix4::identity());
        assert_matrix_eq(&(inv * m), &Matrix4::identity());
    }

    #[test]
    fn test_inverse_singular() {
        let mut m = sample();
        m[3] = m[0];
        assert!(m.inverse().is_none());
        assert_eq!(m.determinant(), 0.0);
    }
}
