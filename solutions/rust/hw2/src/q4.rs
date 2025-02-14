use crate::math::approx::round_coefficients;
use crate::math::interpolation::{interpolate_polynomial, ndd};

#[derive(Debug)]
struct DataPoints {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl DataPoints {
    fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self, &'static str> {
        if !Self::is_valid(&x, &y) {
            return Err("x and y must have the same length");
        }
        Ok(DataPoints { x, y })
    }

    fn is_valid(x: &Vec<f64>, y: &Vec<f64>) -> bool {
        x.len() == y.len()
    }
}

fn solve_p4(data: &DataPoints) {
    let d = ndd(&data.x, &data.y);
    let d_r = round_coefficients(&d, 1e-2);

    let t = 1.5;
    let n = data.x.len();
    let f_t = interpolate_polynomial(&data.x, &d_r, &t, &n);

    println!("Polynomial coefficients: {:?}", &d_r[1..]);
    println!("Interpolated value at t = {t}: {f_t}");
}

pub fn main() {
    let x: Vec<f64> = vec![-2., -1., 0., 1., 2., 3.];
    let y: Vec<f64> = vec![7., 2., 1., 0., 2., 2.];

    match DataPoints::new(x, y) {
        Ok(data) => solve_p4(&data),
        Err(e) => println!("Error creating DataPoints: {}", e),
    }
}
