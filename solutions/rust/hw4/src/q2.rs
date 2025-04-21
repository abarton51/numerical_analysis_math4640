fn f1(x: f64) -> f64 {
    x.exp() - 3. * x.powi(2)
}

fn grad_f1(x: f64) -> f64 {
    x.exp() - 6. * x
}

fn f2(x: f64) -> f64 {
    x - 1. - 0.2 * x.sin()
}

fn grad_f2(x: f64) -> f64 {
    1. - 0.2 * x.cos()
}

fn g(x: f64) -> f64 {
    x.powi(4) + 2. * x + 1.
}

fn grad_g(x: f64) -> f64 {
    4. * x.powi(3) + 2.
}

fn grad_grad_g(x: f64) -> f64 {
    12. * x.powi(2)
}

// x_{n+1} = x_n - (\grad f(x_n))^{-1} f(x_n)
fn newton_method(
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
    x: f64,
    m: usize,
    delta: f64,
    eps: f64,
) -> Option<f64> {
    let mut x0: f64 = x;
    let mut v: f64 = f(x0);
    let mut x1: f64;

    if v.abs() < eps {
        return Some(x0); // root = x0
    }

    for _ in 0..m {
        let dfx0 = df(x0);

        if dfx0.abs() < 1e-15 {
            println!("Derivative too small.");
            return None;
        }

        x1 = x0 - (1. / dfx0) * v;
        v = f(x1);

        if (x1 - x0).abs() < delta || v.abs() < eps {
            return Some(x1);
        }
        x0 = x1;
    }

    println!("Did not converge");
    None
}

fn solve_q2() -> Result<(), Box<dyn std::error::Error>> {
    // f1: e^x + 3x^2 = 0 -> root of f1
    if let Some(root1) = newton_method(f1, grad_f1, -1.0, 100, 1e-10, 1e-10) {
        println!("Root of f1: x = {}", root1);
    }

    // f2: x = 1 + 0.2sin(x) -> root of f2(x) = x - 1 - 0.2sin(x)
    if let Some(root2) = newton_method(f2, grad_f2, 1.0, 100, 1e-10, 1e-10) {
        println!("Root of f2: x = {}", root2);
    }

    // Minimizer of G(x): root of grad_g(x)
    if let Some(minimizer) = newton_method(grad_g, grad_grad_g, -0.5, 10, 1e-10, 1e-10) {
        println!("Minimizer of G(x): x = {}", minimizer);
    }
    Ok(())
}

pub fn main() {
    if let Err(e) = solve_q2() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
