extern crate rand; // Import the rand crate for generating random numbers
use rand::Rng; // Use the Rng trait for random number generation
use std::f64;

// Function to convert binary to decimal
pub fn convert_bin_to_dec(bin_code: &str) -> f64 {
    let mut result = 0.0;
    let mut bin_code = bin_code.trim();

    let is_neg = bin_code.starts_with('-');
    if is_neg {
        bin_code = &bin_code[1..];
    }

    let bins: Vec<&str> = bin_code.split('.').collect();
    let (left_bits, right_bits) = if bins.len() == 2 {
        (bins[0], bins[1])
    } else {
        (bins[0], "")
    };

    let lnum = left_bits.len();

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

    if is_neg {
        result = -result;
    }

    result
}

pub fn convert_dec_to_bin(dec: f64) -> String {
    let mut result = String::new();

    if dec < 0.0 {
        result.push('-');
    }

    let mut integer_part = dec.abs().trunc() as u64;
    let mut fractional_part = dec.abs().fract();

    // Convert the integer part to binary
    if integer_part == 0 {
        result.push('0');
    } else {
        let mut integer_binary = String::new();
        while integer_part > 0 {
            integer_binary.insert(0, if integer_part % 2 == 0 { '0' } else { '1' });
            integer_part /= 2;
        }
        result.push_str(&integer_binary);
    }

    result.push('.');

    let max_iterations = 64;
    let epsilon = 1e-15;
    let mut iteration_count = 0;

    // Convert the fractional part to binary
    while fractional_part > epsilon && iteration_count < max_iterations {
        fractional_part *= 2.0;
        let integer = fractional_part.trunc() as u64;
        result.push(if integer == 1 { '1' } else { '0' });
        fractional_part -= integer as f64;
        iteration_count += 1;
    }

    result
}

pub fn create_cases(size: usize, low: f64, high: f64) -> (Vec<String>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let mut dec_cases = Vec::with_capacity(size);
    let mut bin_code_cases = Vec::with_capacity(size);

    // Generate random decimal numbers
    for _ in 0..size {
        let dec = rng.gen_range(low..high);
        let rounded_dec = (dec * 100000.0).round() / 100000.0;
        dec_cases.push(rounded_dec);
        bin_code_cases.push(convert_dec_to_bin(rounded_dec));
    }

    (bin_code_cases, dec_cases)
}

pub fn main() {
    // setting up hyperparamters for multiple cases
    let size = 10;
    let low = -10.0;
    let high = 10.0;
    // returns binary encodings as strings (with the radix point) and decimal numbers as floats
    let (bin_code_cases, dec_cases) = create_cases(size, low, high);
    println!("Running Solution for Question 3\n--------------------------\n");

    println!("Running Solution for Question 3.a\n");
    println!("Converting Binary to Decimal code for the following cases:\n");

    for c in bin_code_cases.iter() {
        println!("{}\n", c);
    }

    let mut dec_from_bin = Vec::new();
    for (i, c) in bin_code_cases.iter().enumerate() {
        println!("\n---|Case: {}, Binary number: {} |---", i + 1, c);
        let dec_value = convert_bin_to_dec(c);
        dec_from_bin.push(dec_value);
        println!("{}\n", dec_value);
    }
    println!("---DONE---\n");

    println!("Running Solution for Question 3.b\n");
    println!("Converting Decimal to Binary code for the following cases:\n");

    for c in dec_cases.iter() {
        println!("{}\n", c);
    }

    let mut bin_from_dec = Vec::new();
    for (i, c) in dec_cases.iter().enumerate() {
        println!("\n---|Case: {}, Decimal number: {} |---", i + 1, c);
        let bin_value = convert_dec_to_bin(*c);
        bin_from_dec.push(bin_value.clone());
        println!("{}", &bin_value);
    }
    println!("---DONE---\n");
}
