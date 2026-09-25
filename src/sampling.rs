type PatinaFloat = f64;

// Return the linear interpolation between a and b, with 0 <= x <= 1.
fn lerp(x: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    debug_assert!((0.0..=1.0).contains(&x));

    (1.0 - x) * a + x * b
}

// Return the probability distribution of the linear interpolation.
fn linear_pdf(x: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    // If we're outside the domain of lerp, return 0
    if !(0.0..=1.0).contains(&x) {
        0.0
    } else {
        2.0 * lerp(x, a, b) / (a + b)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::sampling::{lerp, linear_pdf};

    #[test]
    fn linear_interpolation() {
        assert_relative_eq!(0.5, lerp(0.5, 0.0, 1.0));
    }

    #[test]
    fn test_linear_pdf() {
        assert_relative_eq!(1.0, linear_pdf(0.5, 0.0, 1.0))
    }
}
