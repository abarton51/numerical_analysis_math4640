use crate::math::approx::round_coefficients;
use crate::math::interpolation::{interpolate_polynomial, ndd};
use lagrangian_interpolation::lagrange_interpolate;

pub fn main() {
    println!("Solution for question 5\n");
    solve_p5();
}

fn solve_p5() {
    let x = vec![1.0, 1.5, 0.0, 2.0];
    let y = vec![3.0, 3.25, 3.0, 5.0 / 3.0];
    let table: Vec<(f64, f64)> = x
        .iter()
        .zip(y.iter())
        .map(|(&x_i, &y_i)| (x_i, y_i))
        .collect();
    let n = x.len();

    let d = ndd(&x, &y);
    let d_r = round_coefficients(&d, 1e-2);

    let t = 1.5;
    let f_t = interpolate_polynomial(&x, &d, &t, &n);
    let lagrange_t = lagrange_interpolate(&table, 1.5);

    println!("Polynomial coefficients: {:?}", &d_r[1..]);
    println!("At t = {t} p_n(t) = {f_t}");
    println!("Lagrange resulted in p_n(t) = {lagrange_t}");
}
