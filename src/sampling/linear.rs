use crate::sampling::PatinaFloat;

/// Sampling using the inversion method.
///
/// Reference: https://pbr-book.org/4ed/Monte_Carlo_Integration/Sampling_Using_the_Inversion_Method
///
/// The general idea here is that given a continuous probability distribution (PDF), we can
/// integrate it to find the cumulative distribution (CDF). We can then obtain a uniformly
/// distributed random number x. Then we generate a sample by solving the equation x = P(X) for X.
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
// Return a sample of the linear distribution, in the range [0, 1).
fn sample_linear(u: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    // If we're outside the domain of lerp, return 0
    if !(0.0..=1.0).contains(&u) {
        0.0
    } else {
        let x = u * (a + b) / (a + lerp(u, a.sqrt(), b.sqrt()));
        // Make sure we're not returning anything outside [0, 1).
        x.min(1.0 - PatinaFloat::EPSILON)
    }
}

// Return the random sample u that corresponds to the random sample x. This corresponds to
// evaluating the CDF.
fn invert_linear_sample(x: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    x * (a * (2.0 - x) + b * x) / (a + b)
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::sampling::linear::{invert_linear_sample, lerp, linear_pdf, sample_linear};

    #[test]
    fn linear_interpolation() {
        assert_relative_eq!(0.5, lerp(0.5, 0.0, 1.0));
    }

    #[test]
    fn test_linear_pdf() {
        assert_relative_eq!(1.0, linear_pdf(0.5, 0.0, 1.0))
    }

    #[test]
    fn invert_sample_roundtrip() {
        // Going from x -> u -> x should be identity.
        let x = 0.5;
        let u = sample_linear(x, 0.0, 1.0);
        assert_relative_eq!(x, invert_linear_sample(u, 0.0, 1.0));
    }
}
