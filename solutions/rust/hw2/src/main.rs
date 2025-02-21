use std::env;
use std::process;

mod math;
mod q4;

const SEPARATOR_STR: &str = "---------------";

fn run_question(question: &str) {
    match question {
        "q4" => {
            println!("\n{}\nRunning solution to question 4...", SEPARATOR_STR);
            q4::main();
        }
        "q5" => {
            println!("\n{}\nRunning solution to question 5...", SEPARATOR_STR);
        }
        _ => {
            println!(
                "\n{}\nError: Could not find the module {}",
                SEPARATOR_STR, question
            );
        }
    }
}

fn main() {
    println!("---MATH 4640, Numerical Analysis, Homework 2---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw2 <question>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {
        run_question(question);
    }
}
