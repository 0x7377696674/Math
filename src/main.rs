//main
//
#![allow(non_snake_case)]

use math::prelude::*;

fn main() {
    let data = vec![2.0, 1.0, 3.0, 4.0, 3.0, 8.0];
    let row = 2;
    let column = 3;
    let mut A = Matrix::from(data, row, column);

    A.reduce();
}
