mod vector2d;

use std::{io::stdin, str::FromStr};
use vector2d::Vector2D;

fn get_stdin_line() -> String {
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().to_owned()
        },
        _ =>  {
            "".to_owned()
        }
    }

}

fn to_f64(str: String) -> f64 {
    match f64::from_str(&str) {
        Ok(r) => r,
        _ => {
            0f64
        }
    }
}

fn main() {
    println!("V1, x:");
    let x1 = to_f64(get_stdin_line());
    println!("V1, y:");
    let y1 = to_f64(get_stdin_line());
    println!("V2, x:");
    let x2 = to_f64(get_stdin_line());
    println!("V2, y:");
    let y2 = to_f64(get_stdin_line());

    let x = Vector2D::new(x1, y1);
    let y = Vector2D::new(x2, y2);
    let t = x - y;

    println!("{}", t);
}