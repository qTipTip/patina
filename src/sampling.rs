type PatinaFloat = f64;

// Return the linear interpolation between a and b, with 0 <= x <= 1.
fn lerp(x: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    debug_assert!((0.0..=1.0).contains(&x));

    (1.0 - x) * a + x * b
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::sampling::lerp;

    #[test]
    fn linear_interpolation() {
        assert_relative_eq!(0.5, lerp(0.5, 0.0, 1.0));
    }
}
