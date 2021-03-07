
use sdl2::pixels::Color;
use crate::shape::Shape;

pub trait Node {
    fn update(&mut self, delta: f32) -> bool;
    fn draw(&self, delta: f32) -> Box<dyn Shape>;
}

pub struct CanvasColor {
    pub i: u8,
}

impl Node for CanvasColor {
    fn update(&mut self, _: f32) -> bool {
        self.i = (self.i + 1) % 255;
        true
    }

    fn draw(&self, _: f32) -> Box<dyn Shape> {
        Box::new(Color::RGB(self.i, 64, 255 - self.i))
    }
}