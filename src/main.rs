use std::io;

use generate_fibonacci::generate_fibonacci;
mod generate_fibonacci;
fn main() {
    println!("Input the number");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Error read line");

    let number: i32 = number.trim().parse().expect("Invalid number");

    if number < 0 {
        panic!("Number must be greater than 0");
    }

    let result = generate_fibonacci(number);
    println!("Fibonacci of {number} is {result}");
}
