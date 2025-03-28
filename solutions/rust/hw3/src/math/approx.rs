pub fn round_coefficients(d: &[f64], tol: f64) -> Vec<f64> {
    d.iter()
        .map(|&c| {
            let nearest_int = c.round();
            if c.abs() < tol {
                0.0
            } else if (c - nearest_int).abs() < tol {
                nearest_int
            } else {
                (c * 1e5).round() / 1e5
            }
        })
        .collect()
}
