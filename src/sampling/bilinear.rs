use crate::{
    math::{PatinaFloat, Point2D},
    sampling::linear::{invert_linear_sample, lerp, sample_linear},
};

fn bi_lerp(p: Point2D<PatinaFloat>, weights: &[PatinaFloat]) -> PatinaFloat {
    (1.0 - p.x) * (1.0 - p.y) * weights[0]
        + p.x * (1.0 - p.y) * weights[1]
        + p.y * (1.0 - p.x) * weights[2]
        + p.x * p.y * weights[3]
}

/// The bilinear function f(x, y) interpolates between four values $w_i$ at the four corners of the
/// unit square [0, 1]**2. The corresponding pdf for f(x, y) is p(x, y) = 4f(x, y) / (w_0 + w_1 + w_2
/// + w_3).
fn bilinear_pdf(p: Point2D<PatinaFloat>, weights: &[PatinaFloat]) -> PatinaFloat {
    // If point is outside the unit square, return 0.
    if !(0.0..=1.0).contains(&p.x) || !(0.0..=1.0).contains(&p.y) {
        return 0.0;
    };

    // If the denominator is 0, return 1.
    let total: PatinaFloat = weights.iter().sum();
    if total == 0.0 {
        return 1.0;
    };

    // Otherwise, return the bilinear pdf:
    4.0 * bi_lerp(p, weights) / total
}

/// Sample the bilinear probability distribution. We do this by first sample y for a bilinear
/// marginal distribution, then we sample x for a bilinear conditional distribution.
fn sample_bilinear(u: Point2D<PatinaFloat>, weights: &[PatinaFloat]) -> Point2D<PatinaFloat> {
    let mut q = Point2D { x: 0.0, y: 0.0 };
    q.y = sample_linear(u.y, weights[0] + weights[1], weights[2] + weights[3]);
    q.x = sample_linear(
        u.x,
        lerp(q.y, weights[0], weights[2]),
        lerp(q.y, weights[1], weights[3]),
    );

    q
}

// Since bilinear sampling is the composition of two linear samples, we can invert them by applying
// inverses in the opposite order.
fn invert_bilinear_sample(
    p: Point2D<PatinaFloat>,
    weights: &[PatinaFloat],
) -> Point2D<PatinaFloat> {
    Point2D {
        x: invert_linear_sample(
            p.x,
            lerp(p.y, weights[0], weights[2]),
            lerp(p.y, weights[1], weights[3]),
        ),
        y: invert_linear_sample(p.y, weights[0] + weights[1], weights[2] + weights[3]),
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::{
        math::Point2D,
        sampling::bilinear::{bi_lerp, invert_bilinear_sample, sample_bilinear},
    };

    #[test]
    fn reproduce_corners() {
        let weights = [1.0, 2.0, 3.0, 4.0];

        assert_relative_eq!(1.0, bi_lerp(Point2D { x: 0.0, y: 0.0 }, &weights));
        assert_relative_eq!(2.0, bi_lerp(Point2D { x: 1.0, y: 0.0 }, &weights));
        assert_relative_eq!(3.0, bi_lerp(Point2D { x: 0.0, y: 1.0 }, &weights));
        assert_relative_eq!(4.0, bi_lerp(Point2D { x: 1.0, y: 1.0 }, &weights));
    }

    #[test]
    fn sample_invert_roundtrip() {
        let weights = [1.0, 2.0, 3.0, 4.0];
        let u = Point2D { x: 0.5, y: 0.7 };
        let p = sample_bilinear(u, &weights);
        let q = invert_bilinear_sample(p, &weights);
        assert_relative_eq!(q.x, u.x);
        assert_relative_eq!(q.y, u.y);
    }
}
