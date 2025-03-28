use plotters::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use std::f64::consts::{PI, SQRT_2};

fn plot_functions<F>(
    func: F,
    fourier_coeffs: &[Complex<f64>],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f64) -> f64,
{
    std::fs::create_dir_all("plots")?;

    let root = BitMapBackend::new(filename, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = 0.0..2.0 * PI;

    let sample_points = 1000;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for i in 0..=sample_points {
        let x = x_range.start + (x_range.end - x_range.start) * (i as f64 / sample_points as f64);
        let y1 = func1(x);
        let y2 = func2(x);

        min_y = min_y.min(y1).min(y2);
        max_y = max_y.max(y1).max(y2);
    }

    let padding = (max_y - min_y) * 0.1;
    let y_range = (min_y - padding)..(max_y + padding);

    let mut chart = ChartBuilder::on(&root)
        .caption("Function Comparison", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("x")
        .y_desc("f(x) or g(x)")
        .draw()?;

    let points1: Vec<(f64, f64)> = (0..=sample_points)
        .map(|i| {
            let x =
                x_range.start + (x_range.end - x_range.start) * (i as f64 / sample_points as f64);
            (x, func(x))
        })
        .collect();

    let points2: Vec<(f64, f64)> = (0..=sample_points)
        .map(|i| {
            let x =
                x_range.start + (x_range.end - x_range.start) * (i as f64 / sample_points as f64);
            (x, reconstruct_function(&fourier_coeffs, 200, x, 5))
        })
        .collect();

    chart
        .draw_series(LineSeries::new(points1.iter().map(|&(x, y)| (x, y)), &RED))?
        .label("f(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(points2.iter().map(|&(x, y)| (x, y)), &BLUE))?
        .label("g_5(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    println!("Plot saved to {}", filename);
    Ok(())
}

fn func1(x: f64) -> f64 {
    (5.0 * x).sin().powi(2)
}

fn func2(x: f64) -> f64 {
    (3.0 * x).cos().powi(3) + (SQRT_2 * x).cos()
}

fn reconstruct_function(fourier_coeffs: &[Complex<f64>], n: usize, x: f64, m: i32) -> f64 {
    let mut sum = 0.0;
    for j in -m..=m {
        let idx = if j < 0 {
            ((j + n as i32) as usize) % n
        } else {
            j as usize % n
        };
        let coef = fourier_coeffs[idx];
        sum += coef.re * (j as f64 * x).cos() - coef.im * (j as f64 * x).sin();
    }
    sum
}

fn calculate_and_print_fourier_coefficients<F>(
    func: F,
    n: usize,
    description: &str,
    theoretical_values: Option<&[(&str, f64)]>,
    filename: &str,
) where
    F: Fn(f64) -> f64,
{
    let x_k = |k: usize| -> f64 { 2.0 * PI * k as f64 / n as f64 };

    let mut buffer: Vec<Complex<f64>> = (0..n)
        .map(|k| Complex {
            re: func(x_k(k)),
            im: 0.0,
        })
        .collect();

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buffer);

    buffer.iter_mut().for_each(|coef| *coef /= n as f64);

    println!("Fourier coefficients for {}", description);
    println!("--------------------------");

    for j in -100..=100 {
        let idx = if j < 0 {
            ((j + n as i32) as usize) % n // Ensure proper wrapping
        } else {
            j as usize % n
        };

        let threshold = 1e-12;
        if buffer[idx].re.abs() > threshold || buffer[idx].im.abs() > threshold {
            println!(
                "f_hat({:3}): {:.12} + i{:.12}",
                j, buffer[idx].re, buffer[idx].im
            );
        }
    }

    plot_functions(func, &buffer, filename);

    if let Some(values) = theoretical_values {
        println!("\n---Theoretical Values---");
        for (label, value) in values {
            println!("{} = {:.12}", label, value);
        }
        println!("All other coefficients = 0");
    }

    println!("-----------------------------");
}

fn solve_q5() -> Result<(), Box<dyn std::error::Error>> {
    let n = 200;

    let func1_theory = [
        ("f_hat(0)", 0.5),
        ("f_hat(10)", -0.25),
        ("f_hat(-10)", -0.25),
    ];

    calculate_and_print_fourier_coefficients(
        func1,
        n,
        "f(x) = sin^2(5x)",
        Some(&func1_theory),
        "plots/first-func-plot.png",
    );

    calculate_and_print_fourier_coefficients(
        func2,
        n,
        "f(x) = cos^2(3x) + cos(\\sqrt{2}x)",
        None,
        "plots/second-func-plot.png",
    );

    Ok(())
}

pub fn main() {
    if let Err(e) = solve_q5() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
