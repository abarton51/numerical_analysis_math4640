use rand::distributions::Distribution;
use statrs::distribution::Normal;
use std::io;

fn main() {
    // Input from the user
    let (size, mean, variance) = get_user_input();

    let normal = Normal::new(mean, variance.sqrt()).expect("Failed to create Normal distribution");
    let sequence: Vec<f64> = (0..size)
        .map(|_| normal.sample(&mut rand::thread_rng()))
        .collect();
    println!("success: {:?}", sequence);
}

fn get_user_input() -> (usize, f64, f64) {
    // Helper function to read input
    fn read_line(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        input.trim().to_string()
    }

    let size: usize = read_line("Please enter the size of the sequence:")
        .parse()
        .expect("Please enter a valid integer");
    let mean: f64 = read_line("Enter the mean (m):")
        .parse()
        .expect("Please enter a valid real number");
    let variance: f64 = read_line("Enter the variance (sigma):")
        .parse()
        .expect("Please enter a valid real number");

    (size, mean, variance)
}
