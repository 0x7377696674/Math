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

    pub fn multiply(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    pub fn dot(&self, other: &Vector) -> i32 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        x + y + z
    }

    pub fn length(&self) -> String {
        let dot = Self::dot(self, self);

        format!("||v|| = \u{221A}{}", dot)
    }

    pub fn linear_combination(c: i32, mut a: Vector, d: i32, mut b: Vector, e: i32, mut z: Vector) -> Self {
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

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

use std::ops::Mul;

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
