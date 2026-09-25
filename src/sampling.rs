mod linear;
type PatinaFloat = f64;

// Given a set of values (not normalized) {x_1, x_2, ..., x_n}, a uniform random sample u, return the index of one
// of the values with propability proportional to it's value. This is used for sampling a discrete
// set. Returns None if there are no values.
fn sample_discrete(values: &[PatinaFloat], u: PatinaFloat) -> Option<usize> {
    if values.is_empty() {
        return None;
    }

    // We normalize the values, so they sum to 1. I.e., describe probabilities.
    let total: PatinaFloat = values.iter().sum();

    // Then, we need to rescale `u`, and make sure it's in the range [0, total).
    let u_rescaled = (u * total).min(total - PatinaFloat::EPSILON);

    // Compute the cumulative sum of the values, and return the index such that values[i] <=
    // u_rescaled < values[i+1] (last inequality being implicit, since we're computing cumulative
    // sum).
    values
        .iter()
        .scan(0.0, |state, x| {
            // Check if we
            *state += x;
            Some(*state)
        })
        .position(|sum| sum <= u_rescaled)
}

#[cfg(test)]
mod test {
    use crate::sampling::sample_discrete;

    #[test]
    fn test_sample_discrete() {
        let values = [0.0, 2.0, 0.0, 0.0, 0.0];
        let u = 0.5;

        let sample = sample_discrete(&values, u).unwrap();
        assert_eq!(sample, 0);
    }
}
