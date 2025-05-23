use crate::math::approx::round_coefficients;
use crate::math::interpolation::{interpolate_polynomial, ndd};

pub fn main() {
    println!("Solution for question 4\n");
    solve_p4();
}

fn solve_p4() {
    let x = vec![-2.0, -1.0, 0.0, 0.5, 2.0, 3.0];
    let y = vec![-5.0, 1.0, 1.0, 0.625, 7.0, 25.0];
    let n = x.len();

    let d = ndd(&x, &y);
    let d_r = round_coefficients(&d, 1e-2);

    let t = 1.5;
    let f_t = interpolate_polynomial(&x, &d_r, &t, &n);

    println!("Polynomial coefficients: {:?}", &d_r[1..]);
    println!("Interpolated value at t = {t}: {f_t}");
}
