pub fn ndd(x: &[f64], y: &[f64]) -> Vec<f64> {
    println!("Beginning Newton's Divided Difference algorithm...");
    let n = x.len();
    assert_eq!(n, y.len(), "x and y must have the same length.");

    let mut d = y.to_vec();

    println!("Calculating coefficients...");
    for i in 1..n {
        for j in (i..n).rev() {
            d[j] = (d[j] - d[j - 1]) / (x[j] - x[j - i])
        }
    }
    println!("Success!");

    d
}

pub fn interpolate_polynomial(x: &[f64], d: &[f64], t: &f64, n: &usize) -> f64 {
    println!("Beginning interpolation of polynomial at point {t}");
    let mut p = d[*n - 1];
    for i in (0..n - 1).rev() {
        p = d[i] + (t - x[i]) * p
    }
    println!("Interpolation succeeded.");

    p
}
