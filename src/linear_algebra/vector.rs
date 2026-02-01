// Vectors

use std::fmt;

#[derive(Debug)]
pub struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn add(a: Vector, b: Vector) -> Self {
        let x = a.x + b.x;
        let y = a.y + b.y;
        Self {
            x,
            y,
        }
    }

    pub fn multiply(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn combine(c: i32, mut a: Vector, d: i32, mut b: Vector) -> Self {
        a.multiply(c);
        b.multiply(d);

        Self::add(a, b)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {} }}", self.x, self.y)
    }
}
