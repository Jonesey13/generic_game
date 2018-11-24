mod glutin_handler;
use games::GameInput;
use glium::Display;
use glium::glutin::EventsLoop;

pub use self::glutin_handler::GlutinInput;

pub trait WindowHandler {
    fn init(&mut self) {}
    fn receive_input(&mut self, &mut EventsLoop) {}
    fn flush_input(&mut self) {}
    fn is_focused(&self) -> bool { true }
    fn request_close(&self) -> bool { false }
}
