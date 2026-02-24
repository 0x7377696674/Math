//main
//
#![allow(non_snake_case)]

use math::prelude::*;

fn main() {
    let data = vec![2, 1, 3, 4, 3, 8, 1, 2, 5];
    let row = 3;
    let column = 3;
    let mut A = Matrix::from(from_i32(data), row, column);

    A.print();
    A.inverse();
}

fn from_i32(vector: Vec<i32>) -> Vec<f32> {
    vector.iter().map(|&x| x as f32).collect()
}
