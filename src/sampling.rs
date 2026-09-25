type PatinaFloat = f64;

// Return the linear interpolation between a and b, with 0 <= x <= 1.
fn lerp(x: PatinaFloat, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat {
    debug_assert!((0.0..=1.0).contains(&x));

    (1.0 - x) * a + x * b
}
