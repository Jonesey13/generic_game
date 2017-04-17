mod glutin_handler;
use games::GameInput;
use glium::backend::glutin_backend::GlutinFacade;

pub use self::glutin_handler::GlutinInput;

pub trait WindowHandler {
    fn init(&mut self) {}
    fn receive_input(&mut self, &mut GlutinFacade) {}
    fn flush_input(&mut self) {}
    fn is_focused(&self) -> bool { true }
}
