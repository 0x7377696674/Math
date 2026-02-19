// matrix

use std::env::current_exe;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    vector: Vec<f32>,
    row: usize,
    column: usize,
}

impl Matrix {
    pub fn from(vector: Vec<f32>, row: usize, column: usize) -> Self {
        Self {
            vector,
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

        self.vector[index]
    }

    pub fn set(&mut self, x: usize, y: usize, new_value: f32) {
        let index = x * self.column + y;

        self.vector[index] = new_value;
    }

    pub fn row_scale(&self, row: usize, scalar: f32) -> Vec<f32> {
        let mut vector = Vec::new();

        for cols in 0..self.column {
            let value = self.get(row, cols);
            vector.push(value * scalar);
        }

        vector
    }

    pub fn upper_triangular(&mut self) {
        self.print();

        for i in 0..self.row {
            let current_row = i;
            let current_column = i;

            // Get the highest value in current column
            let mut highest_value = self.get(current_column, current_row);
            let mut pivot_row = current_row;

            // Gets the row with the highest value in current column
            for r in current_row+1..self.row {
                let value = self.get(r, current_column);

                if value > highest_value {
                    highest_value = value;
                    pivot_row = r;
                }
            }

            // The row with highest value is swapped to the top
            self.row_swap(current_row, pivot_row);
            self.print();

            // Get pivot value for normalizing later
            let pivot = self.get(current_row, current_column); 

            // Normalize the pivot and also its row
            if self.get(current_row, current_column) != 1.0 {
                let new_row = self.row_scale(current_row, 1.0 / pivot);
                self.row_overwrite(current_row, new_row);
            }
            self.print();

            // Here current_row is scaled to get zeros below the current pivot
            for row in current_row + 1..self.row {
                let factor = self.get(row, current_column);
                let scaled = self.row_scale(current_row, -factor);

                self.row_add(row, scaled);
            }
            self.print();
        }
    }
    
    pub fn augment(&mut self, b: Vec<f32>) {
        let mut new_vector = Vec::new();
        for(i, chunk) in self.vector.chunks(self.column).enumerate() {
            new_vector.extend_from_slice(chunk);
            new_vector.push(b[i]);
        }

        self.vector = new_vector;
        self.column += 1;
    }

    pub fn row_add(&mut self, row: usize, other: Vec<f32>) {
        for col in 0..self.column {
            let self_row = self.get(row, col);
            let other = other[col];

            let new_value = self_row + other;

            self.set(row, col, new_value);
        }
    }

    pub fn print(&self) {
        println!("{}", self);
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

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, val) in self.vector.iter().enumerate() {
            write!(f, "{:.2} ", val)?;
            if (i + 1) % self.column == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
