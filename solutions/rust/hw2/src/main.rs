use std::env;
use std::process;

mod math;
mod q4;

const SEPARATOR_STR: &str = "---------------";

fn run_question(question: &str) {
    match question {
        "q4" => {
            println!("Running solution to question 4...");
            q4::main();
        }
        "q5" => {
            println!("Running solution to question 5...");
        }
        _ => {
            println!("Error: Could not find the module {}", question);
        }
    }
}

fn main() {
    println!("---MATH 4640, Numerical Analysis, Homework 1---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw1 <question>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {
        run_question(question);
    }
}
