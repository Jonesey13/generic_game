pub mod multihandler;
pub mod keyboard;
pub mod mouse;
pub mod joystick;
use games::GameInput;

pub use self::keyboard::KeyboardInput;
pub use self::mouse::MouseInput;
pub use self::joystick::JoystickInput;
pub use self::joystick::HatSwitch;

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
