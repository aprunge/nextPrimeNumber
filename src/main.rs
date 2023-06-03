use std::io::{stdout, Write};
use std::process;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f32).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn next_prime_number_generator() {
    let mut number = 2;
    let mut loop_status = true;
    let mut input: String = "".to_string();

    while loop_status {
        let mut prime_found = false; // Move this line inside the while loop
        while !prime_found {
            if is_prime(number) {
                prime_found = true;
                println!("Prime number found: {}", number);
            }
            number += 1;
        }

        input.clear(); // Clear the input variable
        print!("Do you want to find another prime? ");
        stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something bad happened :( not utf-8??");

        if input.trim() == "n" {
            process::exit(0);
        } else {
            loop_status = true;
        }
    }
}

fn main() {
    next_prime_number_generator();
}
