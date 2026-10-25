// Light as carried along rays: the radiance a sample contributes, accumulated by the film and
// scaled by BSDFs and lights. Used throughout the renderer, so it has its own module rather than
// living in any one of its users.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use approx::{AbsDiffEq, RelativeEq};

use crate::math::PatinaFloat;

/// The number of values in a SampledSpectrum. For now these are RGB components. Later w'll replace
/// this with 'N' wavelengths, which represent that wavelengths we're interested in.
pub const N_SPECTRUM_SAMPLES: usize = 3;

/// A spectral quantity, such as radiance, sampled at N_SPECTRUM_SAMPLES points.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct SampledSpectrum {
    values: [PatinaFloat; N_SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
    pub fn new(values: [PatinaFloat; N_SPECTRUM_SAMPLES]) -> Self {
        Self { values }
    }

    /// The same value at every sample
    pub fn constant(c: PatinaFloat) -> Self {
        Self::new([c; N_SPECTRUM_SAMPLES])
    }

    /// Whether every value is zero
    pub fn is_black(&self) -> bool {
        self.values.iter().all(|&v| v == 0.0)
    }

    pub fn max_component_value(&self) -> PatinaFloat {
        self.values
            .iter()
            .copied()
            .fold(PatinaFloat::NEG_INFINITY, PatinaFloat::max)
    }

    pub fn min_component_value(&self) -> PatinaFloat {
        self.values
            .iter()
            .copied()
            .fold(PatinaFloat::INFINITY, PatinaFloat::min)
    }

    pub fn average(&self) -> PatinaFloat {
        self.values.iter().sum::<PatinaFloat>() / N_SPECTRUM_SAMPLES as PatinaFloat
    }

    /// Whether any value is NaN.
    pub fn has_nan(&self) -> bool {
        self.values.iter().any(|v| v.is_nan())
    }

    // Applies `f` to every value.
    fn map(self, f: impl Fn(PatinaFloat) -> PatinaFloat) -> Self {
        Self::new(self.values.map(f))
    }

    // Combines the values of two spectra pairwise with `f`.
    fn zip_with(self, rhs: Self, f: impl Fn(PatinaFloat, PatinaFloat) -> PatinaFloat) -> Self {
        Self::new(std::array::from_fn(|i| f(self.values[i], rhs.values[i])))
    }
}

impl Index<usize> for SampledSpectrum {
    type Output = PatinaFloat;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for SampledSpectrum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

// Add two spectra together
impl Add for SampledSpectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |a, b| a + b)
    }
}

// Subtract two spectra
impl Sub for SampledSpectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |a, b| a - b)
    }
}

// Multiply two spectra
impl Mul for SampledSpectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |a, b| a * b)
    }
}

// Divide a spectrum by another
impl Div for SampledSpectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        debug_assert!(
            rhs.values.iter().all(|&v| v != 0.0),
            "division by a spectrum with a zero value"
        );
        self.zip_with(rhs, |a, b| a / b)
    }
}

impl AddAssign for SampledSpectrum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for SampledSpectrum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for SampledSpectrum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for SampledSpectrum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for SampledSpectrum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(|v| -v)
    }
}

// Sclaar multiplication
impl Mul<PatinaFloat> for SampledSpectrum {
    type Output = Self;
    fn mul(self, rhs: PatinaFloat) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

impl Mul<SampledSpectrum> for PatinaFloat {
    type Output = SampledSpectrum;
    fn mul(self, rhs: SampledSpectrum) -> Self::Output {
        rhs * self
    }
}
// Scalar division
impl Div<PatinaFloat> for SampledSpectrum {
    type Output = Self;
    fn div(self, rhs: PatinaFloat) -> Self::Output {
        debug_assert!(rhs != 0.0, "division of a spectrum by zero");
        self.map(|v| v / rhs)
    }
}

impl MulAssign<PatinaFloat> for SampledSpectrum {
    fn mul_assign(&mut self, rhs: PatinaFloat) {
        *self = *self * rhs;
    }
}

impl DivAssign<PatinaFloat> for SampledSpectrum {
    fn div_assign(&mut self, rhs: PatinaFloat) {
        *self = *self / rhs;
    }
}

// Eq-operators
impl AbsDiffEq for SampledSpectrum {
    type Epsilon = PatinaFloat;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.values
            .iter()
            .zip(other.values)
            .all(|(a, b)| a.abs_diff_eq(&b, epsilon))
    }
}

impl RelativeEq for SampledSpectrum {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-12
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.values
            .iter()
            .zip(other.values)
            .all(|(a, b)| a.relative_eq(&b, epsilon, max_relative))
    }
}

#[cfg(test)]
mod test_sampled_spectrum {
    use approx::assert_relative_eq;

    use crate::spectrum::SampledSpectrum;

    fn s(r: f64, g: f64, b: f64) -> SampledSpectrum {
        SampledSpectrum::new([r, g, b])
    }

    #[test]
    fn test_spectrum_arithmetic_is_component_wise() {
        let a = s(1.0, 2.0, 3.0);
        let b = s(0.5, 4.0, -1.0);

        assert_eq!(a + b, s(1.5, 6.0, 2.0));
        assert_eq!(a - b, s(0.5, -2.0, 4.0));
        assert_eq!(a * b, s(0.5, 8.0, -3.0));
        assert_eq!(a / b, s(2.0, 0.5, -3.0));
        assert_eq!(-a, s(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_scalar_arithmetic() {
        let a = s(1.0, 2.0, 3.0);

        assert_eq!(a * 2.0, s(2.0, 4.0, 6.0));
        assert_eq!(2.0 * a, a * 2.0);
        assert_eq!(a / 2.0, s(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_assign_operators_match_binary_operators() {
        let a = s(1.0, 2.0, 3.0);
        let b = s(0.5, 4.0, -1.0);

        let mut c = a;
        c += b;
        assert_eq!(c, a + b);
        c -= b;
        assert_eq!(c, a);
        c *= b;
        assert_eq!(c, a * b);
        c /= b;
        assert_relative_eq!(c, a);
        c *= 3.0;
        assert_eq!(c, a * 3.0);
        c /= 3.0;
        assert_relative_eq!(c, a);
    }

    #[test]
    fn test_queries() {
        let a = s(1.0, -2.0, 4.0);

        assert!(!a.is_black());
        assert!(SampledSpectrum::default().is_black());
        assert!(SampledSpectrum::constant(0.0).is_black());
        assert_eq!(a.max_component_value(), 4.0);
        assert_eq!(a.min_component_value(), -2.0);
        assert_relative_eq!(a.average(), 1.0);
        assert!(!a.has_nan());
        assert!(s(1.0, f64::NAN, 0.0).has_nan());
    }

    #[test]
    fn test_indexing() {
        let mut a = s(1.0, 2.0, 3.0);
        a[1] = 5.0;

        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 5.0);
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero_value_panics_in_debug() {
        let _ = s(1.0, 1.0, 1.0) / s(1.0, 0.0, 1.0);
    }
}
