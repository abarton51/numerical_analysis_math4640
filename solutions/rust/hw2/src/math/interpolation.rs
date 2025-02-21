// Newton's Divided Difference Algorithm
pub fn ndd(x: &[f64], y: &[f64]) -> Vec<f64> {
    println!("Beginning Newton's Divided Difference algorithm...");
    let n = x.len();
    assert_eq!(n, y.len(), "x and y must have the same length.");

    // Div Diff table
    let mut d = vec![vec![0.0; n]; n];

    // First column is y
    for i in 0..n {
        d[i][0] = y[i];
    }

    println!("Calculating divided difference table...");
    for j in 1..n {
        for i in 0..(n - j) {
            let numerator: f64 = d[i + 1][j - 1] - d[i][j - 1];
            let denominator: f64 = x[i + j] - x[i];
            d[i][j] = numerator / denominator;
            println!(
                "f[x{}, ..., x{}] = ({} - {}) / ({} - {}) = {}\n",
                i,
                i + j,
                d[i + 1][j - 1],
                d[i][j - 1],
                x[i + j],
                x[i],
                d[i][j]
            );
        }
    }
    println!("Success! Divided difference table:");
    for row in 0..n {
        println!("{:?}", d[row]);
    }

    d[0].clone()
}

// Interpolation given NDD coefficients
// p(x) = d[0] + (x - x[0])*d[1] + (x - x[1])*(x - x[0])*d[2] + ...
pub fn interpolate_polynomial(x: &[f64], d: &[f64], t: &f64, n: &usize) -> f64 {
    println!("Beginning interpolation of polynomial at point {t}");
    let mut p = d[*n - 1];
    for i in (0..*n - 1).rev() {
        p = d[i] + (t - x[i]) * p
    }
    println!("Interpolation succeeded.");

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_quadratic_happy() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let y: Vec<f64> = vec![0.0, -1.0, 0.0];
        let n = x.len();
        let d: Vec<f64> = ndd(&x, &y);

        let expected_d: Vec<f64> = vec![0.0, -1.0, 1.0];

        for (computed, expected) in d.iter().zip(expected_d.iter()) {
            assert!(
                (computed - expected).abs() < 1e-10,
                "Expected {}, got {}",
                expected,
                computed
            );
        }

        let test_points = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        println!("\nTesting interpolation at various points:");
        for &t in test_points.iter() {
            let interpolated = interpolate_polynomial(&x, &d, &t, &n);
            // f(x) = x^2 - 4x + 3
            let actual = t * t - 4.0 * t + 3.0;

            println!(
                "f({}) = {} (interpolated) vs {} (actual)",
                t, interpolated, actual
            );
            assert!(
                (interpolated - actual).abs() < 1e-10,
                "At x = {}, expected {}, got {}",
                t,
                actual,
                interpolated
            );
        }
    }
}
