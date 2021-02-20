mod vector2d;

use std::{io::stdin, str::FromStr};
use vector2d::Vector2D;

extern crate sdl2;

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

#[allow(dead_code)]
fn run_newton_run() {
    let m1:f64 = 1.989e+30;
    let m2:f64 = 5.972e+24;
    let v1 = Vector2D::new(
        0.,
        0.,
    );
    let v2 = Vector2D::new(
        -149.96e9,
        0.,
    );
    let r21_v = v2 - v1;
    let dist = r21_v.norm_abs();
    println!("{}", &dist);
    let g = 6.674e-11;
    
    let f = (g*m1*m2) / (dist*dist);
    let f_v = -f * (r21_v/dist);
    println!("dist\t= {}\nF\t= {}\nF21->\t= {}\nF21->2\t= {}\nacc(m1)\t= {}\nacc(m2)\t= {}", dist, f, f_v, (-g*m1*m2*r21_v)/dist/dist/dist, f_v/m1, f_v/m2);
}

fn main() {
    let _sdl = match sdl2::init() {
        Ok(s) => s,
        Err(err) => panic!("Problem initializing sdl: {:?}", err),
    };
}