use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub trait Shape {
    fn draw(&self, canvas: &mut WindowCanvas) -> Result<bool, String>;
}

impl Shape for Color {
    fn draw(&self, canvas: &mut WindowCanvas) -> Result<bool, String> {
        canvas.set_draw_color(*self);
        canvas.clear();

        Ok(true)
    }
}