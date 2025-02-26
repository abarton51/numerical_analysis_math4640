use core::f64;

use crate::math::approx::round_coefficients;
use crate::math::interpolation::{interpolate_polynomial, ndd};
use plotters::prelude::*;
use rand::Rng;

fn plot_interpolation_results(
    r: &[f64],
    f_r: &[f64],
    p_r: &[f64],
    error: &[f64],
    x_k: &[f64],
    f_x_k: &[f64],
    degree: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename: String = format!("interpolation_plot_degree_{}.png", degree);
    let root = BitMapBackend::new(&filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(600);

    let y_max = f_r
        .iter()
        .chain(p_r.iter())
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let y_min = f_r
        .iter()
        .chain(p_r.iter())
        .fold(f64::INFINITY, |a, &b| a.min(b));

    let mut main_chart = ChartBuilder::on(&upper)
        .caption(
            "Function vs. Interpolated Polynomial",
            ("sans-serif", 30).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5f64..5f64, (y_min - 0.1)..(y_max + 0.1))?;

    main_chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("y")
        .x_labels(11)
        .y_labels(21)
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()?;

    main_chart
        .draw_series(
            x_k.iter()
                .zip(f_x_k.iter())
                .map(|(&x, &y)| Circle::new((x, y), 5, RED.filled())),
        )?
        .label("Interpolation points")
        .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    main_chart
        .draw_series(
            r.iter()
                .zip(f_r.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, BLUE.mix(0.3))),
        )?
        .label("f(x)")
        .legend(|(x, y)| Circle::new((x, y), 5, BLUE.filled()));

    main_chart
        .draw_series(
            r.iter()
                .zip(p_r.iter())
                .map(|(&x, &p)| Circle::new((x, p), 2, GREEN.mix(0.3))),
        )?
        .label("p(x) (interpolated polynomial)")
        .legend(|(x, y)| Circle::new((x, y), 5, GREEN.filled()));

    main_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    let error_max: f64 = error.iter().fold(0.0, |a, &b| a.max(b));

    let mut error_chart = ChartBuilder::on(&lower)
        .caption("Error", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5f64..5f64, 0f64..(error_max + error_max * 0.1))?;

    error_chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("Error")
        .draw()?;

    error_chart
        .draw_series(
            r.iter()
                .zip(error.iter())
                .map(|(&x, &e)| Circle::new((x, e), 2, RED.mix(0.5).filled())),
        )?
        .label("Absolute Error")
        .legend(|(x, y)| Circle::new((x, y), 2, RED.mix(0.5).filled()));

    error_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;

    Ok(())
}

fn function_f(x: f64) -> f64 {
    2. / (1. + x.powi(2))
}

fn evaluate_on_rand(
    x_k: &[f64],
    d: &[f64],
    num_points: i32,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut p_random_points: Vec<f64> = Vec::new();

    let mut error: Vec<f64> = Vec::new();

    let mut rng = rand::rng();
    let random_points: Vec<f64> = (0..num_points)
        .map(|_| rng.random_range(-5.0..=5.0))
        .collect();

    let f_random_points: Vec<f64> = random_points.iter().map(|&r| function_f(r)).collect();

    for (i, &t) in random_points.iter().enumerate() {
        p_random_points.push(interpolate_polynomial(&x_k, &d, &t, &x_k.len()));
        error.push((p_random_points[i] - f_random_points[i]).abs());
    }

    (random_points, f_random_points, p_random_points, error)
}

fn solve_q5(a: f64, b: f64, n: i32) {
    let step = (b - a) / (n + 2) as f64;
    println!("Polynomial will have degree of {}", n);
    let x_k: Vec<f64> = (1..n + 2).map(|i| a + (i as f64) * step).collect();

    println!("Evenly spaced points on [{}, {}]: {:?}", a, b, x_k);

    let f_x_k: Vec<f64> = x_k.iter().map(|&x| function_f(x)).collect();

    println!("Data Table:\nx = {:?}\nf(x) = {:?}", x_k, f_x_k);

    let d = ndd(&x_k, &f_x_k);
    println!(
        "\nNewton Divided Difference coefficients for polynomial of degree {}: {:?}",
        n, d
    );

    let (random_points, f_random_points, p_random_points, error): (
        Vec<f64>,
        Vec<f64>,
        Vec<f64>,
        Vec<f64>,
    ) = evaluate_on_rand(&x_k, &d, 2000);

    match plot_interpolation_results(
        &random_points,
        &f_random_points,
        &p_random_points,
        &error,
        &x_k,
        &f_x_k,
        n,
    ) {
        Ok(_) => println!("Plot saved as `interpolation_plot_{}.png`", n),
        Err(e) => println!("Failed to create plot: {}", e),
    }
}

pub fn main() {
    solve_q5(-5., 5., 2);

    solve_q5(-5., 5., 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation() {
        let a = -5.0;
        let b = 5.0;
        let n = 9;
        let step = (b - a) / (n + 1) as f64;
        let x_k: Vec<f64> = (0..n + 1).map(|i| a + (i as f64) * step).collect();

        let f_x_k: Vec<f64> = x_k.iter().map(|&x| function_f(x)).collect();

        let d = ndd(&x_k, &f_x_k);

        for (i, &t) in x_k.iter().enumerate() {
            let f_t = interpolate_polynomial(&x_k, &d, &t, &x_k.len());
            assert!((f_x_k[i] - f_t).abs() < 1e-4);
        }
    }
}
