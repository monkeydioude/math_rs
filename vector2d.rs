use std::{fmt::{Display, Formatter, Result}, ops::{Add, Div, Mul, Sub}};

// Vector2D is a simple 2 dimensional struct
// for force computation.
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D {x, y}
    }

    // norm_abs gives the absolute version
    // of the norm
    pub fn norm_abs(&self) -> f64 {
        f64::sqrt((self.x*self.x)+(self.y*self.y))
    }
}

impl Copy for Vector2D {}
impl Clone for Vector2D {
    fn clone(&self) -> Vector2D {
        *self
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

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

// Add<Vector2D> allows "+" (sum) operation over a Vector2D and an f64 
impl Add<Vector2D> for f64 {
    type Output = Vector2D;

    fn add (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self + other.x, self + other.y)
    }
}

// Add<f64> allows "+" (sum) operation over an f64 and a Vector2D
impl Add<f64> for Vector2D {
    type Output = Vector2D;

    fn add (self, v: f64) -> Vector2D {
        Vector2D::new(self.x + v, self.y + v)
    }
}

// Mul<Vector2D> allows "*" (multiply) operation over 2 Vector2D
impl Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

// Mul<Vector2D> allows "*" (multiply) operation over a Vector2D and an f64 
impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self * other.x, self * other.y)
    }
}

// Mul<f64> allows "*" (multiply) operation over a f64 and a Vector2D
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul (self, v: f64) -> Vector2D {
        Vector2D::new(self.x * v, self.y * v)
    }
}

// Div<f64> allows "/" (divide) operation over a f64 and a Vector2D
impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, v: f64) -> Vector2D {
        if v == 0f64 {
            return Vector2D::new(0f64, 0f64)
        }
        Vector2D::new(self.x / v, self.y / v)
    }
}

// Div<f64> allows "/" (divide) operation over a f64 and a borrowed Vector2D
impl Div<f64> for &Vector2D {
    type Output = Vector2D;

    fn div(self, v: f64) -> Vector2D {
        if v == 0f64 {
            return Vector2D::new(0f64, 0f64)
        }
        Vector2D::new(self.x / v, self.y / v)
    }
}