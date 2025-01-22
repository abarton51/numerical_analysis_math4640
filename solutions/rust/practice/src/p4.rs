pub fn main() {
    let result = iter_factorial(&5);
    println!("Result: {result}")
}

fn iter_factorial(n: &u32) -> u64 {
    let mut result: u64 = 1;
    for i in 2..*n + 1 {
        result *= i as u64;
    }
    result
}

fn rec_factorial(n: &i64) -> i64 {
    if *n == 0 {
        return 1;
    }
    n * rec_factorial(&(n - 1))
}
