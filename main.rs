use std::{fmt::{Display, Formatter, Result}, io::stdin, ops::{Add, Mul, Sub}};

// Vector2D is a simple 2 dimensional struct
// for force computation.
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
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

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

// Add<Vector2D> allows "+" operation over a Vector2D and an f64 
impl Add<Vector2D> for f64 {
    type Output = Vector2D;

    fn add (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self + other.x, self + other.y)
    }
}

// Add<f64> allows "+" operation over an f64 and a Vector2D
impl Add<f64> for Vector2D {
    type Output = Vector2D;

    fn add (self, v: f64) -> Vector2D {
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

// Mul<Vector2D> allows "*" operation over a Vector2D and an f64 
impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul (self, other: Vector2D) -> Vector2D {
        Vector2D::new(self * other.x, self * other.y)
    }
}

// Mul<f64> allows "*" operation over an i64 and a Vector2D
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul (self, v: f64) -> Vector2D {
        Vector2D::new(self.x * v, self.y * v)
    }
}


fn get_stdin_line() -> Option<String> {
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            Some(input.trim().to_owned())
        },
        _ =>  {
            println!("WESH !!");
            None
        }
    }

}

fn to_f64(str: String) -> Option<f64> {
    use std::str::FromStr;

    match f64::from_str(&str) {
        Ok(r) => Some(r),
        _ => {
            println!("WESH !!");
            None
        }
    }
}

fn main() {
    println!("V1, x:");
    let x1 = to_f64(get_stdin_line().unwrap()).unwrap();
    println!("V1, y:");
    let y1 = to_f64(get_stdin_line().unwrap()).unwrap();
    println!("V2, x:");
    let x2 = to_f64(get_stdin_line().unwrap()).unwrap();
    println!("V2, y:");
    let y2 = to_f64(get_stdin_line().unwrap()).unwrap();

    let x = Vector2D::new(x1, y1);
    let y = Vector2D::new(x2, y2);
    let t = x - y;

    println!("{}", t);
}