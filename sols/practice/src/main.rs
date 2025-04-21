use std::env;
use std::process;

mod p2;
mod p3;
mod p4;

fn run_question(question: &str) {
    match question {
        "p2" => {
            println!("Running q2 module...\n");
            p2::main();
        }
        "p4" => {
            println!("Running q4 module...\n");
            p4::main();
        }
        "all" => {
            println!("Running all solutions...\n");
            let v: Vec<f64> = p2::main();
            let sorted_v = p3::sort_seq(&v, false);
            println!("sorted_v: {:?}", sorted_v);
            p4::main();
        }
        _ => {
            println!("Error: Could not find the module {}", question);
        }
    }
}

fn main() {
    println!("---MATH 4640, Numerical Analysis, PRACTICE---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <[p]ractice_question[#]>");
        process::exit(1);
    }

    for question in args.iter().skip(1) {
        run_question(question);
    }
}
