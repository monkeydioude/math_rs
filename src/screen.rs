extern crate sdl2;

pub struct Screen {
    pub canvas: sdl2::render::WindowCanvas,
    pub sdl_context: sdl2::Sdl,
}

impl Screen {
    pub fn new(w: u32, h:u32, title: &str) -> Screen {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
 
        let window = video_subsystem.window(title, w, h)
            .position_centered()
            .build()
            .unwrap();
        Screen{
            canvas: window.into_canvas().build().unwrap(),
            sdl_context,
        }
    }
}