pub mod multihandler;
pub mod keyboard;
pub mod mouse;
use games::GameInput;

pub trait InputHandler {
    fn init(&mut self) {}
    fn receive_input(&mut self) {}
    fn pass_on_input<'a>(&self, Option<&'a mut GameInput>) {}
    fn escape_key_pressed(&self) -> bool { false }
    fn flush_input(&mut self) {}
}

#[allow(dead_code)]
pub struct InputHandlerStub;

impl InputHandler for InputHandlerStub {}
