use std::{fmt::{Display, Formatter, Result}, ops::{Add, Mul}};
// Vector2D is a simple 2 dimensional struct
// for force computation.
struct Vector2D {
    x: i64,
    y: i64,
}

impl Vector2D {
    pub fn new(x: i64, y: i64) -> Vector2D {
        Vector2D {x, y}
    }
}

// Display implementation allows to display the struct on std output
impl Display for Vector2D {
    fn fmt(&self, format: &mut Formatter<'_>) -> Result {
        format
            .debug_struct("Vector2D")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// Add<Vector2D> allows "+" operation over 2 Vector2D
impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

// Add<Vector2D> allows "+" operation over a Vector2D and an i64 
impl Add<Vector2D> for i64 {
    type Output = Vector2D;

    fn add (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self + other.x, self + other.y)
    }
}

// Add<Vector2D> allows "+" operation over an i64 and a Vector2D
impl Add<i64> for Vector2D {
    type Output = Vector2D;

    fn add (self, v: i64) -> Vector2D {
        Vector2D::new(self.x + v, self.y + v)
    }
}

// Mul<Vector2D> allows "*" operation over 2 Vector2D
impl Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

// Mul<Vector2D> allows "*" operation over a Vector2D and an i64 
impl Mul<Vector2D> for i64 {
    type Output = Vector2D;

    fn mul (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self * other.x, self * other.y)
    }
}

// Mul<Vector2D> allows "*" operation over an i64 and a Vector2D
impl Mul<i64> for Vector2D {
    type Output = Vector2D;

    fn mul (self, v: i64) -> Vector2D {
        Vector2D::new(self.x * v, self.y * v)
    }
}


fn main() {
    let a: i64 = 3;
    let b: i64 = 2;
    let c = Vector2D::new(2, 3);

    let t = (a + b) * c * 2;

    println!("{}", t);
}