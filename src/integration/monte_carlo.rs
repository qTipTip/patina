use crate::math::PatinaFloat;
use rand::{self, RngExt};
// Returns n uniformly spaced values between a and b, inclusive.
fn linspace(n: usize, a: PatinaFloat, b: PatinaFloat) -> (Vec<PatinaFloat>, PatinaFloat) {
    let delta = (b - a) / (n as PatinaFloat - 1.0);
    (
        (0..n).map(|i| a + i as PatinaFloat * delta).collect(),
        delta,
    )
}

// Integrate the function f from a to b using n uniform monte carlo samples. Not to be confused
// with monte carlo integration with importance sampling.
//
// Here, we draw N uniformly distributed samples from [0, 1].
// - We use these to sample N uniformly
// distributed points x_i in the interval [a, b].
// - We then compute the integral of f from a to b by
// computing the value of f(x_i) at each point
// - Finally, computing the average by summing and then scaling by (1 / (b -
// a)) (which is the constant p(x) from the uniform distribution).
fn uniform_integrate_mc<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat
where
    F: Fn(&PatinaFloat) -> PatinaFloat,
{
    let mut rng = rand::rng();
    let uniform_dist = rand::distr::Uniform::new(a, b).unwrap();
    (0..n)
        .map(|_| {
            let x = rng.sample(uniform_dist);
            f(&x)
        })
        .sum::<PatinaFloat>()
        / (n as PatinaFloat * (b - a))
}

fn integrate_mc_importance_sampling<F>(
    n: usize,
    f: F,
    a: PatinaFloat,
    b: PatinaFloat,
    pdf: F,
) -> PatinaFloat
where
    F: Fn(&PatinaFloat) -> PatinaFloat,
{
    0.0
}

// Integrate the function f from a to b using n uniformly spaced values, and the trapezoidal rule.
fn integrate_trapezoidal<F>(n: usize, f: F, a: PatinaFloat, b: PatinaFloat) -> PatinaFloat
where
    F: Fn(&PatinaFloat) -> PatinaFloat,
{
    let (x_vals, delta) = linspace(n, a, b);
    let f_vals: Vec<PatinaFloat> = x_vals.iter().map(f).collect();

    let mut sum = 0.0;
    for i in 0..n - 1 {
        sum += f_vals[i + 1] + f_vals[i]
    }

    sum * delta / 2.0
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::{
        integration::monte_carlo::{integrate_trapezoidal, linspace, uniform_integrate_mc},
        math::PatinaFloat,
    };

    #[test]
    fn mc_x_squared() {
        let n = 10000;
        let a = 0.0;
        let b = 1.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        let (x, _) = linspace(n, a, b);
        let y: Vec<PatinaFloat> = x.iter().map(f).collect();

        assert_eq!(x.len(), n);
        assert_relative_eq!(y[0], 0.0);
        assert_relative_eq!(y[y.len() - 1], 1.0);

        assert_relative_eq!(
            uniform_integrate_mc(n, f, a, b),
            1.0 / 3.0,
            epsilon = 1.0e-2
        );
    }

    #[test]
    fn tpz_x_squared() {
        let n = 10000;
        let a = 0.0;
        let b = 1.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        assert_relative_eq!(
            integrate_trapezoidal(n, f, a, b),
            1.0 / 3.0,
            epsilon = 1.0e-5
        );
    }
}
