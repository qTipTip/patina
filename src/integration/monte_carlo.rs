use crate::{
    math::PatinaFloat,
    sampling::linear::{linear_pdf, sample_linear},
};
use rand::{self, RngExt};
// Returns n uniformly spaced values between a and b, inclusive.
fn linspace(n: usize, a: PatinaFloat, b: PatinaFloat) -> Vec<PatinaFloat> {
    (0..n)
        .map(|i| a + i as PatinaFloat * (b - a) / (n as PatinaFloat - 1.0))
        .collect()
}

// Integrate the function f from a to b using n monte carlo samples.
fn integrate_mc<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat
where
    F: Fn(&PatinaFloat) -> PatinaFloat,
{
    let mut rng = rand::rng();

    // Generate n canonically uniformly distributed random variables, and sample the linear
    // distribution. Also keep track of the probability of sampling that particular value.
    let samples: Vec<(PatinaFloat, PatinaFloat)> = (0..n)
        .map(|_| {
            let u = rng.random();

            let x = sample_linear(u, a, b);
            let p = linear_pdf(x, a, b);
            (x, p)
        })
        .collect();

    // We can then compute the monte carlo estimator to f by evaluating f at each sampled point,
    // divide it by the probability of choosing that particular point, and average all values at
    // the end.
    samples.iter().map(|(x, p)| f(x) / p).sum::<PatinaFloat>() / n as PatinaFloat
}

fn integrate_rk4<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat)
where
    F: Fn(PatinaFloat) -> PatinaFloat,
{
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::{
        integration::monte_carlo::{integrate_mc, linspace},
        math::PatinaFloat,
    };

    #[test]
    fn fk4_x_squared() {
        let n = 1000;
        let a = -2.0;
        let b = 3.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        let x = linspace(n, a, b);
        let y: Vec<PatinaFloat> = x.iter().map(f).collect();

        assert_eq!(x.len(), n);
        assert_relative_eq!(y[0], 4.0);
        assert_relative_eq!(y[y.len() - 1], 9.0);

        assert_relative_eq!(integrate_mc(1000, f, a, b), 2.0 / 3.0);
    }
}
