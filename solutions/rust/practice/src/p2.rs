use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use std::io;

pub fn main() {
    let (size, mean, variance) = get_user_input();
    let mut rng = thread_rng();
    let normal = Normal::new(mean, variance).expect("Failed to make normal distribution");
    for _ in 0..size {
        let v = normal.sample(&mut rng);
        println!("{} is from a N({}, {}) distribution", v, mean, variance);
    }
}

fn get_user_input() -> (usize, f64, f64) {
    fn read_line(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    let size: usize = read_line("Enter size of sequence:")
        .parse()
        .expect("Please enter valid integer");
    let mean: f64 = read_line("Enter the mean (m):")
        .parse()
        .expect("Please enter a valid number");
    let variance: f64 = read_line("Enter the variance (sigma):")
        .parse()
        .expect("Please enter a valid number");
    (size, mean, variance)
}
