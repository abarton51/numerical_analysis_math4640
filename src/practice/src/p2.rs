use rand;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use std::io;

pub fn main() -> Vec<f64> {
    let (size, mean, variance) = get_user_input();
    // walkthrough(size, mean, variance);
    let v: Vec<f64> = solution(size, mean, variance.sqrt());
    println!("Solution to question 2: {:?}", v);
    v
}

fn solution(size: usize, mean: f64, variance: f64) -> Vec<f64> {
    let rng = thread_rng();
    let normal = Normal::new(mean, variance).expect("Failed to create Normal in solution");
    let v: Vec<f64> = normal.sample_iter(rng).take(size).collect();
    v
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
    let variance: f64 = read_line("Enter the std (sigma):")
        .parse()
        .expect("Please enter a valid number");
    (size, mean, variance)
}
/*
fn walkthrough(size: usize, mean: f64, variance: f64) -> () {
    let mut rng = thread_rng();
    println!("init normal distribution from rand_distr crate");
    let normal = Normal::new(mean, variance.sqrt()).expect("Failed to make normal distribution");
    println!("We can iterate and sample each iteration from the normal distribution we created:");
    for _ in 0..size {
        let v = normal.sample(&mut rng);
        println!("{} is from a N({}, {}) distribution", v, mean, variance);
    }
    println!("\nFor a simple distribution from `rand::distributions` we can use sample_iter(rng). Note this is not the normal distribution before, we're just showing a better implementation for generating a sequence of samples rather than a loop.");
    let v: Vec<f32> = rand::distributions::Standard
        .sample_iter(&mut rng)
        .take(size)
        .collect();
    println!("{:?}\n", v);
    println!("Similarly for distributions from rand_distr:");
    let v: Vec<f64> = normal.sample_iter(&mut rng).take(size).collect();
    println!("{:?}\n", v);

    println!("rand::dsitributions provides simple basic random generation. It's good for uniform distributions. statrs and rand_distr are better for more generic stuff. Make sure the Distribution trait is imported from the rand or rand_distr crate, as each distribution implements that trait.");
}
*/
