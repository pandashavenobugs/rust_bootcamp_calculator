use std::io;

enum Calculator {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(calc: Calculator) -> f64 {
    match calc {
        Calculator::Add(a, b) => a + b,
        Calculator::Subtract(a, b) => a - b,
        Calculator::Multiply(a, b) => a * b,
        Calculator::Divide(a, b) => a / b,
    }
}
fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut operation = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: f64 = x.trim().parse().expect("Please type a number!");

    println!("Enter the operation: ");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y: f64 = y.trim().parse().expect("Please type a number!");

    let operation = operation.trim();
    let result = match operation {
        "+" => calculate(Calculator::Add(x, y)),
        "-" => calculate(Calculator::Subtract(x, y)),
        "*" => calculate(Calculator::Multiply(x, y)),
        "/" => calculate(Calculator::Divide(x, y)),
        _ => panic!("Invalid operation"),
    };
    println!("The result is: {}", result);
}
