// Vectors

use std::fmt;

#[derive(Debug)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn add(a: Vector, b: Vector) -> Self {
        let x = a.x + b.x;
        let y = a.y + b.y;
        let z = a.z + b.z;
        Self {
            x,
            y,
            z,
        }
    }

    pub fn multiply(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    pub fn dot(a: Vector, b: Vector) -> i32 {
        let x = a.x * b.x;
        let y = a.y * b.y;
        let z = a.z * b.z;

        x + y + z
    }

    pub fn combine(c: i32, mut a: Vector, d: i32, mut b: Vector, e: i32, mut z: Vector) -> Self {
        a.multiply(c);
        b.multiply(d);
        z.multiply(e);

        let result = Self::add(a, b);
        Self::add(result, z)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {} }}", self.x, self.y)
    }
}

use std::ops::Add;

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
