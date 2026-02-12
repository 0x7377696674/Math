// Vectors

use std::fmt;

#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn info(&self, other: Self) {
        let dot = self.dot(&other);

        if dot > 0.0 {
            println!("angle is less than 90");
        } else if dot < 0.0 {
            println!("angle is more than 90");
        } else {
            println!("angle is 90");
        }
    }

    pub fn unit(&self) -> Self {
        let dot = self.dot(self);

        Self {
            x: self.x / dot.sqrt(),
            y: self.y / dot.sqrt(),
            z: self.z / dot.sqrt(),
        }
    }

    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        x + y + z
    }

    pub fn length(&self) -> String {
        let dot = Self::dot(self, self);

        format!("||v|| = \u{221A}{}", dot)
    }

    pub fn linear_combination(c: f64, mut x: Vector, d: f64, mut y: Vector, e: f64, mut z: Vector) -> Self {
        x.scale(c);
        y.scale(d);
        z.scale(e);

        x + y + z
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
