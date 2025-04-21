use std::env;
use std::process;

mod math;
mod q5;
mod q8;

const SEPARATOR_STR: &str = "---------------";

fn run_question(question: &str) {
    match question {
        "q5" => {
            println!("\n{}\nRunning solution to question 5...", SEPARATOR_STR);
            q5::main();
            println!("\n{}\nEnd of solution to question 5 :)", SEPARATOR_STR);
        }
        "q8" => {
            println!("\n{}\nRunning solution to question 8...", SEPARATOR_STR);
            q8::main();
            println!("\n{}\nEnd of solution to question 8 :)", SEPARATOR_STR);
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
    println!("---MATH 4640, Numerical Analysis, Homework 3---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw3 <question>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {
        run_question(question);
    }
}
