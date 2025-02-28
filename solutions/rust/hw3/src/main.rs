use std::env;
use std::process;

const SEPARATOR_STR: &str = "----------------";

fn main() {
    println!("---MATH 4640, Numerical Analysis, Homework 3---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hw2 <question>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {}
}
