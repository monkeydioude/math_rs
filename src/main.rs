mod vector2d;
pub mod shape;
pub mod screen;
pub mod node;

use std::{io::stdin, str::FromStr};

extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use screen::Screen;
use node::{Node, CanvasColor};

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
    let screen = Screen::new(800, 600, "pouet, pouet");
    let mut canvas = screen.canvas;
    let mut event_pump = screen.sdl_context.event_pump().unwrap();
    let mut nodes: Vec<Box<dyn Node>> = Vec::new();
    nodes.push(Box::new(CanvasColor{i: 0}));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let mut to_rm: Vec<usize> = Vec::new();
        for (it, node) in nodes.iter_mut().enumerate() {
            let res = node.update(0.0);
            if res == Ok(false) {
                to_rm.push(it);
            }
        }

        for e in to_rm.iter() {
            nodes.remove(*e);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}