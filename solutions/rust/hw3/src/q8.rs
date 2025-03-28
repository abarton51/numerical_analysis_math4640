use std::f64::consts::PI;

fn comp_trap_rule(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));

    for i in 1..n {
        let x = a + (i as f64) * h;
        sum += f(x);
    }

    h * sum
}

pub fn main() {
    let f1 = |x: f64| (-x.powi(2) / 2.0).exp();
    let f2 = |x: f64| 1.0 / (2.0 + x.sin());

    let a1 = 0.0;
    let b1 = 1.0;

    let a2 = 0.0;
    let b2 = 2.0 * PI;

    println!("a) integral from 0 to 1 of e^(-x^2 / 2) dx");
    for n in [128, 256, 512] {
        let result = comp_trap_rule(f1, a1, b1, n);
        println!("n = {}: {:.16}", n, result);
    }

    println!("b) integral from 0 to 2pi of 1/(2 + sin(x) dx");
    for n in [128, 256, 512] {
        let result = comp_trap_rule(f2, a2, b2, n);
        println!("n = {}: {:.16}", n, result);
    }
}
