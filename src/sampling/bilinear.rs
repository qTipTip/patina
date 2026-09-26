use crate::math::{PatinaFloat, Point2D};

fn bi_lerp(p: Point2D, weights: &[PatinaFloat]) -> PatinaFloat {
    (1.0 - p.x) * (1.0 - p.y) * weights[0]
        + p.x * (1.0 - p.y) * weights[1]
        + p.y * (1.0 - p.x) * weights[2]
        + p.x * p.y * weights[3]
}

/// The bilinear function f(x, y) interpolates between four values $w_i$ at the four corners of the
/// unit square [0, 1]**2. The corresponding pdf for f(x, y) is p(x, y) = 4f(x) / (w_0 + w_1 + w_2
/// + w_3).
fn bilinear_pdf(p: Point2D, weights: &[PatinaFloat]) -> PatinaFloat {
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
