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
    let mut r0 = a;
    let mut r1 = b;

    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;

    // I won't handle exceptions for now, I'll suppose input will always be right!

    while r1 != 0 {
        let quotient = r0 / r1;

        let r2 = r1;
        r1 = r0 - quotient * r1;
        r0 = r2;

        let s2 = s1;
        s1 = s0 - quotient * s1;
        s0 = s2;

        let t2 = t1;
        t1 = t0 - quotient * t1;
        t0 = t2;
    }

    println!("Bezout coefficients: {} {}", s0, t0);
    println!("Greatest common divisor: {}", r0);
    println!("Quotients bt the gcd: {} {}", t1, s1);
}
