use std::{fmt::{Display, Formatter, Result}, ops::{Add, Div, Mul, Sub}};

// Vector2D is a simple 2 dimensional struct
// for force computation.
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::Vector2D;

    #[test]
    fn i_can_add_2_vector2d() {
        let goal = Vector2D::new(3., 3.);
        let trial = Vector2D::new(1., 1.) + Vector2D::new(2., 2.);
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_add_f64_w_vector2d() {
        let goal = Vector2D::new(69., 420.);
        let trial = Vector2D::new(36., 387.) + 33.;
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_multiply_2_vector2d() {
        let goal = Vector2D::new(1_870.869_000_000_000_1, 2.742_9);
        let trial = Vector2D::new(4.45, 1.23) * Vector2D::new(420.42, 2.23);
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_multiply_f64_w_vector2d() {
        let goal = Vector2D::new(96.024, 420.69);
        let trial = 33. * Vector2D::new(2.9098181818181818181818181818182, 12.748181818181818181818181818182);
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_substract_2_vector2d() {
        let goal = Vector2D::new(2.5, -25.);
        let trial = Vector2D::new(12., 50.) - Vector2D::new(9.5, 75.);
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_divide_vector2d_w_f64() {
        let goal = Vector2D::new(45., 0.25);
        let trial = Vector2D::new(180., 1.) / 4.;
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }

    #[test]
    fn i_can_compute_norm_abs() {
        let goal = 5.;
        let trial = Vector2D::new(4., -3.);
        assert_eq!(trial.norm_abs(), goal);
    }

    #[test]
    fn i_can_be_the_new_newton_ton() {
        let goal = Vector2D::new(35252499978477410000000.0,-0.0);
        let r21_v = Vector2D::new(-149.96e9, 0.) - Vector2D::new(0., 0.);
        let dist = r21_v.norm_abs();
        let trial = (-6.674e-11*1.989e+30*5.972e+24*r21_v)/dist/dist/dist;
        assert_eq!(trial.x, goal.x);
        assert_eq!(trial.y, goal.y);
    }
}