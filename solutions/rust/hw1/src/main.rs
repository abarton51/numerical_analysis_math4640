use std::env;
use std::process;

mod q3; // Include solution for question 3

fn run_question(question: &str) {
    match question {
        "q3" => q3::main(),
        _ => println!("Error: Could not find the module {}", question),
    }
}

fn main() {
    println!("---MATH 4640, Numerical Analysis, Homework 1---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw1 -q <question>");
        process::exit(1);
    }
    let question = &args[1];

    println!("Running question: {}", question);
    run_question(question);
}
