// Newton's DivDif
pub fn ndd(x: &[f64], y: &[f64]) -> Vec<f64> {
    println!("Beginning Newton's Divided Difference algorithm...");
    let n = x.len();
    assert_eq!(n, y.len(), "x and y must have the same length.");

    let mut d = vec![vec![0.0; n]; n];

    // first column is y
    for i in 0..n {
        d[i][0] = y[i];
    }

    println!("Calculating coefficients...");
    for j in 1..n {
        for i in 0..(n - j) {
            let numerator: f64 = d[i + 1][j - 1] - d[i][j - 1];
            let denominator: f64 = x[i + j] - x[i];
            d[i][j] = numerator / denominator;
        }
    }
    println!("Success! {:?}", d);

    d[0].clone()
}

//for i in 1..n {
//    for j in (i..n).rev() {
//        d[j] = (d[j] - d[j - 1]) / (x[j] - x[j - i]);
//    }
//}

// Interp
pub fn interpolate_polynomial(x: &[f64], d: &[f64], t: &f64, n: &usize) -> f64 {
    println!("Beginning interpolation of polynomial at point {t}");
    let mut p = d[*n - 1];
    for i in (0..n - 1).rev() {
        p = d[i] + (t - x[i]) * p
    }
    println!("Interpolation succeeded.");

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ndd_works() {
        let x: Vec<f64> = vec![-2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
        let y: Vec<f64> = vec![7.0, 2.0, 1.0, 0.0, 2.0, 2.0];
        let d: Vec<f64> = ndd(&x, &y);
        let expected_d: Vec<f64> = vec![
            1.0,
            -17.0 / 12.0,
            -7.0 / 24.0,
            13.0 / 24.0,
            7.0 / 24.0,
            -1.0 / 8.0,
        ];
        for (computed, expected) in d.iter().zip(expected_d.iter()) {
            assert!(
                (computed - expected).abs() < 1e-10,
                "Expected {}, got {}",
                expected,
                computed
            );
        }
    }
}
