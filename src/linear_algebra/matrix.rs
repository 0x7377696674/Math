// matrix

use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    data: Vec<f32>,
    row: usize,
    column: usize,
}

impl Matrix {
    pub fn from(data: Vec<f32>, row: usize, column: usize) -> Self {
        Self {
            data,
            row,
            column,
        }
    }

    pub fn size(&self) -> usize {
        self.row * self.column
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {

        // Row Major Formula
        // Index = rows * num_of_columns + columns
        let index = x * self.column + y;

        self.data[index]
    }

    pub fn set(&mut self, x: usize, y: usize, new_value: f32) {
        let index = x * self.column + y;

        self.data[index] = new_value;
    }

    pub fn row_scale(&self, row: usize, scalar: f32) -> Vec<f32> {
        let mut vector = Vec::new();

        for cols in 0..self.column {
            let value = self.get(row, cols);
            vector.push(value * scalar);
        }

        vector
    }

    pub fn gauss(&mut self) {
        for current_column in 0..self.column {
            let current_row = 0;

            if (current_row + 1) * (current_column + 1) == self.size() {
                break;
            }

            // Get the highest value in the column
            let mut highest_value = self.get(current_column, current_row);
            let mut pivot_row = current_row;

            // Loops to find the row with the highest column value
            for r in 0..self.row {
                let value = self.get(r, current_column);

                if value > highest_value {
                    highest_value = value;
                    pivot_row = r;
                }
            }

            // If everything goes fine, I can swap current row with the row with highest column value
            self.row_swap(current_row, pivot_row);

            // Now, I should check if pivot is 1, if isn't I should scale the whole row by 1 / pivot
            let pivot = self.get(current_row, current_column); 

            // Here pivot is turned into 1
            if self.get(current_row, current_column) != 1.0 {
                let new_row = self.row_scale(current_row, 1.0 / pivot);
                self.row_overwrite(current_row, new_row);
            }
            dbg!(&self);
        }
    }

    pub fn row_add(&mut self, row: usize, other: Vec<f32>) {
        for col in 0..self.column {
            let self_row = self.get(row, col);
            let other = other[col];

            let new_value = self_row + other;

            self.set(row, col, new_value);
        }
    }

    pub fn row_swap(&mut self, a: usize, b: usize) {
        let row_a = self.row(a);
        let row_b = self.row(b);

        self.row_overwrite(a, row_b);
        self.row_overwrite(b, row_a);
    } 

    pub fn row_overwrite(&mut self, row: usize, new_row: Vec<f32>) {
        for i in 0..self.column {
            self.set(row, i, new_row[i]);
        }
    }

    pub fn row(&self, x: usize) -> Vec<f32> {
        let mut n = Vec::new();

        for y in 0..self.column {
            let value = self.get(x, y);
            n.push(value);
        }

        n
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let row = self.row;
        let column = rhs.column;

        let mut vector = Vec::new();

        for a in 0..self.row {
            for b in 0..rhs.column {
                let mut value = 0.0;

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
