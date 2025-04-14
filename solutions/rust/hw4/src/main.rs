use std::env;
use std::process;

mod q2;

const SEPARATOR_STR: &str = "---------------";

fn run_question(question: &str) {
    match question {
        "q2" => {
            println!("\n{}\nRunning solution to question 2...", SEPARATOR_STR);
            q2::main();
            println!("\n{}\nEnd of solution to question 2 :)", SEPARATOR_STR);
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
    println!("---MATH 4640, Numerical Analysis, Homework 4---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw4 <question>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {
        run_question(question);
    }
}
