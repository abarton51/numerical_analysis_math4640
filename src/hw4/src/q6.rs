use plotters::prelude::*;

fn gauss_seidel_iteration(
    a: &[Vec<f64>],
    b: &[f64],
    exact: &[f64],
    max_iter: usize,
    tol: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = b.len();

    let mut x_old = vec![0.0; n];
    let mut x_new = vec![0.0; n];

    let mut errors: Vec<(usize, f64)> = Vec::new();

    for k in 0..max_iter {
        for i in 0..n {
            let mut sigma_1 = 0.0;
            let mut sigma_2 = 0.0;
            for j in 0..n {
                if j < i {
                    sigma_1 += a[i][j] * x_new[j]
                }
                if j > i {
                    sigma_2 += a[i][j] * x_old[j];
                }
            }
            x_new[i] = (b[i] - sigma_1 - sigma_2) / a[i][i];
        }

        let error_norm = x_new
            .iter()
            .zip(exact)
            .map(|(xi, ei)| (xi - ei).abs())
            .fold(0.0, f64::max);

        errors.push((k, error_norm));

        let diff: f64 = x_new
            .iter()
            .zip(&x_old)
            .map(|(x_new_i, x_old_i)| (x_new_i - x_old_i).abs())
            .fold(0.0, f64::max);

        if diff < tol {
            println!("Converged after {} iterations.", k + 1);
            break;
        }

        x_old.copy_from_slice(&x_new);
    }

    // Plotting
    std::fs::create_dir_all("plots")?;
    let root =
        BitMapBackend::new("plots/gauss_seidel_error_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_error = errors.iter().map(|&(_, e)| e).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Gauss Seidel Iteration: Error Norm vs Iteration",
            ("sans-serif", 25),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..errors.len(), 0.0..max_error * 1.1)?;

    chart
        .configure_mesh()
        .x_desc("Iteration")
        .y_desc("Infinity Norm of Error")
        .draw()?;

    chart
        .draw_series(LineSeries::new(errors.clone(), &BLUE))?
        .label("Error Norm")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 15, y)], &BLUE));

    chart
        .configure_series_labels()
        .border_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Plot saved to plots/gauss_seidel_error_plot.png");

    println!("Computed solution x:");
    for xi in x_new {
        println!("{:.6}", xi);
    }
    Ok(())
}

fn jacobi_iteration(
    a: &[Vec<f64>],
    b: &[f64],
    exact: &[f64],
    max_iter: usize,
    tol: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = b.len();

    let mut x_old = vec![0.0; n];
    let mut x_new = vec![0.0; n];

    let mut errors: Vec<(usize, f64)> = Vec::new();

    for k in 0..max_iter {
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if j != i {
                    sigma += a[i][j] * x_old[j];
                }
            }
            x_new[i] = (b[i] - sigma) / a[i][i];
        }

        let error_norm = x_new
            .iter()
            .zip(exact)
            .map(|(xi, ei)| (xi - ei).abs())
            .fold(0.0, f64::max);

        errors.push((k, error_norm));

        let diff: f64 = x_new
            .iter()
            .zip(&x_old)
            .map(|(x_new_i, x_old_i)| (x_new_i - x_old_i).abs())
            .fold(0.0, f64::max);

        if diff < tol {
            println!("Converged after {} iterations.", k + 1);
            break;
        }

        x_old.copy_from_slice(&x_new);
    }

    // Plotting
    std::fs::create_dir_all("plots")?;
    let root = BitMapBackend::new("plots/jacobi_error_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_error = errors.iter().map(|&(_, e)| e).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Jacobi Iteration: Error Norm vs Iteration",
            ("sans-serif", 25),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..errors.len(), 0.0..max_error * 1.1)?;

    chart
        .configure_mesh()
        .x_desc("Iteration")
        .y_desc("Infinity Norm of Error")
        .draw()?;

    chart
        .draw_series(LineSeries::new(errors.clone(), &BLUE))?
        .label("Error Norm")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 15, y)], &BLUE));

    chart
        .configure_series_labels()
        .border_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Plot saved to plots/jacobi_error_plot.png");

    println!("Computed solution x:");
    for xi in x_new {
        println!("{:.6}", xi);
    }
    Ok(())
}

fn solve_q6() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<Vec<f64>> = vec![
        vec![4., -1., 0., -1., 0., 0.],
        vec![-1., 4., -1., 0., -1., 0.],
        vec![0., -1., 4., 0., 0., -1.],
        vec![-1., 0., 0., 4., -1., 0.],
        vec![0., -1., 0., -1., 4., -1.],
        vec![0., 0., -1., 0., -1., 4.],
    ];

    let b: Vec<f64> = vec![2., 1., 2., 2., 1., 2.];

    let exact: Vec<f64> = vec![1., 1., 1., 1., 1., 1.];

    let tol: f64 = 1e-12;
    let max_iter = 1000;

    jacobi_iteration(&a, &b, &exact, max_iter, tol)?;

    gauss_seidel_iteration(&a, &b, &exact, max_iter, tol)?;

    Ok(())
}

pub fn main() {
    if let Err(e) = solve_q6() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
