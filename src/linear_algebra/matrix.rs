// matrix

use std::ops::{Add, AddAssign, Mul, Div};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Matrix<T> {
    pub vector: Vec<T>,
    pub row: u32,
    pub column: u32,
}

impl<T> Matrix<T> {
    pub fn from(vector: Vec<T>, row: u32, column: u32) -> Self {
        Self {
            vector,
            row,
            column,
        }
    }

    pub fn size(&self) -> u32 {
        self.row * self.column
    }

    pub fn get(&self, x: u32, y: u32) -> T 
    where 
        T: Copy,
    {
        // Row Major Formula
        // Index = rows * num_of_columns + columns
        let index = x as i32 * self.column as i32 + y as i32;

        self.vector[index as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, new_value: T)
    {
        let index = x as i32 * self.column as i32 + y as i32;

        self.vector[index as usize] = new_value;
    }

    pub fn identity(mut a: Matrix<T>)
    // This doesn't work yet!
    where 
        T: Copy + Display + Debug + Mul<Output = T>,
    {
        let row = 0;
        let column = 0;

        let pivot = a.get(row, column);
        println!("Pivot in ({row}{column}): {pivot}");

        Self::scale(&mut a, row, pivot);
        println!("New A: {a:#?}");
    }

    pub fn scale(a: &mut Matrix<T>, row: u32, pivot: T)
    where 
        T: Copy + Mul<Output = T>,
    {
        for y in 0..a.column {
            let value = a.get(row, y);
            a.set(row, y, value * pivot);
        }
    }
}

impl<T> Mul for Matrix<T>
where 
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + Debug + AddAssign,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let row = self.row;
        let column = rhs.column;

        let mut vector = Vec::new();

        dbg!(self.row, rhs.column, self.column);

        for a in 0..self.row {
            for b in 0..rhs.column {
                let mut value = T::default();

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
