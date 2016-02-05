pub mod multihandler;
pub mod keyboard;
use games::GameInput;

pub trait InputHandler {
    fn init(&mut self) {}
    fn receive_input(&mut self) {}
    fn pass_on_input<'a>(&self, Option<&'a mut GameInput>) {}
    fn escape_key_pressed(&self) -> bool { false }
}

#[allow(dead_code)]
pub struct InputHandlerStub;

impl InputHandler for InputHandlerStub {}
