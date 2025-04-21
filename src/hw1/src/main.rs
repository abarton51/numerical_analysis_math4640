use std::env;
use std::process;

mod math;
mod q3; // Include solution for question 3
mod q4;
mod q5;

const SEPARATOR_STR: &str = "---------------";

fn run_question(question: &str) {
    match question {
        "q3" => {
            println!("{SEPARATOR_STR}\nRunning q3 module...\n{SEPARATOR_STR}\n");
            q3::main();
        }
        "q4" => {
            println!("{SEPARATOR_STR}\nRunning q4 module...\n{SEPARATOR_STR}\n");
            q4::main();
        }
        "q5" => {
            println!("{SEPARATOR_STR}\nRunning q5 module...\n{SEPARATOR_STR}\n");
            q5::main();
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
