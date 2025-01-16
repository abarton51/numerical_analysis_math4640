extern crate rand; // Import the rand crate for generating random numbers
use rand::Rng; // Use the Rng trait for random number generation
use std::f64;

// Function to convert binary to decimal
pub fn convert_bin_to_dec(bin_code: &str) -> f64 {
    let mut result = 0.0;
    let bins: Vec<&str> = bin_code.split('.').collect();
    let (left_bits, right_bits) = (bins[0], bins[1]);
    let lnum = left_bits.len();
    let rnum = right_bits.len();

    // Process left bits (integer part)
    for (i, bit) in left_bits.chars().enumerate() {
        if bit == '1' {
            result += 2.0f64.powi((lnum - (i + 1)) as i32);
        }
    }

    // Process right bits (fraction part)
    for (j, bit) in right_bits.chars().enumerate() {
        if bit == '1' {
            result += 2.0f64.powi(-(j as i32 + 1));
        }
    }

    result
}

pub fn convert_dec_to_bin(mut dec: f64) -> String {
    let mut result = String::new();
    let mut integer_part = dec.trunc() as u64;
    let mut fractional_part = dec.fract();

    // Convert the integer part to binary
    while integer_part > 0 {
        result.insert(0, if integer_part % 2 == 0 { '0' } else { '1' });
    }

    result.push('.');

    result
}

pub fn create_cases(size: usize, low: f64, high: f64) -> (Vec<String>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let mut dec_cases = Vec::with_capacity(size);
    let mut bin_code_cases = Vec::with_capacity(size);

    // Generate random decimal numbers
    for _ in 0..size {
        let dec = rng.gen_range(low..high);
        dec_cases.push(dec);
    }

    (bin_code_cases, dec_cases)
}

pub fn main() {
    println!("Running Solution for Question 1\n");

    println!("---DONE---\n");
}
