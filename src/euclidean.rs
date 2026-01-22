// greatest common divisor (GCD)

use std::mem;

pub fn euclids_algorithm(mut a: i32, mut b: i32) -> i32 {
    loop {
        if a == b {
            println!("The greatest common divisor is {}", a);
            return a;
        }

        if a > b {
            a -= b;
        } else {
            mem::swap(&mut a, &mut b);
        }
    }
}

pub fn extended_euclids_algorithm(a: i32, b: i32) {
    /// This function expects A > B
    assert!(a > b, "A must be bigger than B");

    /// Formula: x = x1 - quotient * x2

    /// Turns the input A and B into mutable
    let mut x1 = a;
    let mut x2 = b;

    loop {
        // Counts the iteration loops
        let mut iteration = 0;

        if x2 == 0 {
            println!("The greatest commom divisor of {a} and {b} is {x1}");
            return;
        }

        let quotient = x1 / x2;
        let remainder = x1 - quotient * x2;

        x1 = x2;
        x2 = remainder;

        println!(
            "ITERATION: {}\nx1: {x1}\nx2: {x2}\nquotient: {quotient}\nremainder: {remainder}\n",
            iteration
        );

        iteration += 1;
    }
}
