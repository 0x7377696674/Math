// greatest common divisor (GCD)

use std::mem::swap;

pub fn euclids_algorithm(mut a: i32, mut b: i32) -> i32 {
    loop {
        if a == b {
            return a;
        }

        if a > b {
            a -= b;
        } else {
            swap(&mut a, &mut b);
        }
    }
}
