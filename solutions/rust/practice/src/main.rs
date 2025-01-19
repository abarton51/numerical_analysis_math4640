use std::env;
use std::process;

mod p2; // Include solution for question 2
mod p3;

fn run_question(question: &str) {
    match question {
        "p2" => {
            println!("Running q3 module...\n");
            p2::main();
        }
        "all" => {
            println!("Running all solutions...\n");
            let v: Vec<f64> = p2::main();
            let sorted_v = p3::sort_seq(&v, false);
            println!("sorted_v: {:?}", sorted_v);
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
