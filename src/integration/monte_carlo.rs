use crate::math::PatinaFloat;
use rand::{self, RngExt, distr};
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
// - Finally, compute the average and scale it by (b - a), the reciprocal of the
// uniform density p(x) = 1 / (b - a).
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
        / (n as PatinaFloat)
        * (b - a)
}

// While `uniform_integrate_mc` assumes the uniform distribution of the sampled values, using
// importance sampling, we can sample _more_ points where the function f is large, and less
// elsewhere. This is a variance reduction technique. The probability distribution `pdf` can be
// chosen arbitrarily, but is usually chosen to "look like" f.
fn integrate_mc_importance_sampling<F, P, S>(
    n: usize,
    f: F,
    a: PatinaFloat,
    b: PatinaFloat,
    pdf: P,
    sample_pdf: S,
) -> PatinaFloat
where
    F: Fn(PatinaFloat) -> PatinaFloat,
    P: Fn(PatinaFloat) -> PatinaFloat,
    // Takes a uniform sample [0, 1), and transforms it according to the target distribution
    S: Fn(PatinaFloat) -> PatinaFloat,
{
    let mut rng = rand::rng();
    let uniform_dist = distr::Uniform::new(0.0, 1.0).unwrap();

    let mut sum = 0.0;
    for _ in 0..n {
        // Get a canonical uniformly distributed sample
        let u = rng.sample(uniform_dist);
        // Transform the uniform sample into the target distribution
        let x = sample_pdf(u);

        // If we're within the domain
        if x >= a && x <= b {
            // Compute the probability of having chosen x
            let p = pdf(x);
            // If probability is positive, compute the corresponding value and weight it by the probability
            if p > 0.0 {
                sum += f(x) / p;
            }
        }
    }
    sum / (n as PatinaFloat)
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
        integration::monte_carlo::{
            integrate_mc_importance_sampling, integrate_trapezoidal, linspace, uniform_integrate_mc,
        },
        math::PatinaFloat,
    };

    #[test]
    fn mc_x_squared() {
        let n = 1000000;
        let a = 0.0;
        let b = 2.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        let (x, _) = linspace(n, a, b);
        let y: Vec<PatinaFloat> = x.iter().map(f).collect();

        assert_eq!(x.len(), n);
        assert_relative_eq!(y[0], 0.0);
        assert_relative_eq!(y[y.len() - 1], 4.0);

        assert_relative_eq!(
            uniform_integrate_mc(n, f, a, b),
            8.0 / 3.0,
            epsilon = 1.0e-2
        );
    }

    #[test]
    fn tpz_x_squared() {
        let n = 100000;
        let a = 0.0;
        let b = 2.0;

        let f = |x: &PatinaFloat| -> PatinaFloat { x * x };

        assert_relative_eq!(
            integrate_trapezoidal(n, f, a, b),
            8.0 / 3.0,
            epsilon = 1.0e-5
        );
    }

    #[test]
    fn mc_importance_sampling_x_squared() {
        let n = 10000;
        let a = 0.0;
        let b = 2.0;

        let f = |x: PatinaFloat| -> PatinaFloat { x * x };
        // For f(x) = x*x over the interval [0, 2], we choose the PDF(x) = x / 2, as it's integral
        // is 1 over [0, 2].
        let pdf = |x: PatinaFloat| -> PatinaFloat { x / 2.0 };
        // Since the CDF(x) = x^2 / 4, the inverse transform sampling function is x = 2 * sqrt(u).
        let sample_pdf = |u: PatinaFloat| -> PatinaFloat { 2.0 * u.sqrt() };
        assert_relative_eq!(
            integrate_mc_importance_sampling(n, f, a, b, pdf, sample_pdf),
            8.0 / 3.0,
            epsilon = 1.0e-2
        );
    }
}
