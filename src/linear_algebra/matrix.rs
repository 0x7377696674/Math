// matrix

use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    vector: Vec<i32>,
    row: u32,
    column: u32,
}

impl Matrix {
    pub fn from(vector: Vec<i32>, row: u32, column: u32) -> Self {
        Self {
            vector,
            row,
            column,
        }
    }

    pub fn size(&self) -> u32 {
        self.row * self.column
    }

    pub fn get(&self, x: u32, y: u32) -> i32 {

        // Row Major Formula
        // Index = rows * num_of_columns + columns
        let index = x as i32 * self.column as i32 + y as i32;

        self.vector[index as usize]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let row = self.row;
        let column = rhs.column;

        let mut vector = Vec::new();

        dbg!(self.row, rhs.column, self.column);

        for a in 0..self.row {
            for b in 0..rhs.column {
                let mut value = 0;

                for z in 0..self.column {
                    let left = self.get(a, z);
                    let right = rhs.get(z, b);

                    value += left * right;
                    dbg!(value);
                }

                vector.push(value);
            }
        } 

        Self::from(vector, row, column)
    }
}
