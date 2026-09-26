use crate::math::PatinaFloat;

// Integrate the function f from a to b using n monte carlo samples.
fn integrate_mc<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat)
where
    F: Fn(PatinaFloat) -> PatinaFloat,
{
}

fn integrate_rk4<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat)
where
    F: Fn(PatinaFloat) -> PatinaFloat,
{
}

#[cfg(test)]
mod test {
    use approx::{assert_relative_eq, assert_relative_ne};

    use crate::math::PatinaFloat;

    #[test]
    fn fk4_x_squared() {
        let n = 1000;
        let a = -2.0;
        let b = 3.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        let x: Vec<PatinaFloat> = (0..n)
            .map(|i| a + i as PatinaFloat * (b - a) / (n as PatinaFloat - 1.0))
            .collect();
        let y: Vec<PatinaFloat> = x.iter().map(f).collect();

        assert_eq!(x.len(), n);
        assert_relative_eq!(y[0], 4.0);
        assert_relative_eq!(y[y.len() - 1], 9.0);
    }
}
