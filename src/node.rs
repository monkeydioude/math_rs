use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub trait Node {
    fn update(&self, delta: f32) -> Result<bool, String>;
    fn render(&self, canvas: WindowCanvas, delta: f32) -> Result<bool, String>;
}

pub struct CanvasColor {
    i: u8,
}

impl Node for CanvasColor {
    fn update(&self, _: f32) -> Result<bool, String> {
        self.i = (self.i + 1) % 255;
        Ok(true)
    }

    fn render(&self, canvas: WindowCanvas, _: f32) -> Result<bool, String> {
        canvas.set_draw_color(Color::RGB(self.i, 64, 255 - self.i));
        canvas.clear();
        Ok(true)
    }
}