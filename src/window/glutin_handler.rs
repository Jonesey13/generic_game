use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event::*;
use super::WindowHandler;

pub struct GlutinInput {
    focused_flag: bool
}

impl GlutinInput {
    pub fn new() -> GlutinInput {
        GlutinInput {
            focused_flag: true,
        }
    }
}

impl WindowHandler for GlutinInput {
    fn receive_input(&mut self, display: &mut GlutinFacade) {
        for item in display.poll_events() {
            match item
            {
                Focused(b) => self.focused_flag = b,
                _ => (),
            }
        }
    }
    
    fn is_focused(&self) -> bool { self.focused_flag }
}
