mod vector2d;
// mod big_float;

use std::{io::stdin, str::FromStr};
use vector2d::Vector2D;

#[allow(dead_code)]
fn get_stdin_line(msg: &str) -> String {
    let mut input = String::new();

    println!("{}", msg);
    match stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().to_owned()
        },
        _ =>  {
            "".to_owned()
        }
    }

}

#[allow(dead_code)]
fn to_f64(str: String) -> f64 {
    match f64::from_str(&str) {
        Ok(r) => r,
        _ => {
            0f64
        }
    }
}

fn main() {
    let m1:f64 = 5.972e+24;
    let m2:f64 = 1.989e+30;
    // let v1 = Vector2D::new(
    //     to_f64(get_stdin_line("V1, x:")), 
    //     to_f64(get_stdin_line("V1, y:")),
    // );
    // let v2 = Vector2D::new(
    //     to_f64(get_stdin_line("V2, x:")), 
    //     to_f64(get_stdin_line("V2, y:")),
    // );
    let v1 = Vector2D::new(
        750000000000.0, 
        750000000000.0,
    );
    let v2 = Vector2D::new(
        0.0, 
        0.0,
    );
    let r21_v = v2 - v1;
    let dist = r21_v.norm_abs();
    let g = 6.674e-11;
    
    let f = (g*m1*m2) / (dist*dist);
    let f_v = -f * (r21_v/dist);
    println!("dist\t= {}\nF\t= {}\nF21->\t= {}\nF21->2\t= {}\nacc(m1)\t= {}\nacc(m2)\t= {}", dist, f, f_v, (-g*m1*m2*r21_v)/dist/dist/dist, (-g*m2*r21_v)/dist/dist/dist, (-g*m1*r21_v)/dist/dist/dist);
}