pub mod multihandler;
pub mod keyboard;
pub mod mouse;
pub mod joystick;
pub mod bool_switch;
use crate::games::GameInput;

pub use self::keyboard::KeyboardInput;
pub use self::mouse::MouseInput;
pub use self::joystick::JoystickInput;
pub use self::joystick::HatSwitch;

pub trait InputHandler {
    fn init(&mut self) {}
    fn reset(&mut self) {}
    fn receive_input(&mut self) {}
    fn pass_on_input<'a>(&self, _: Option<&'a mut dyn GameInput>) {}
    fn escape_key_pressed(&self) -> bool { false }
    fn backtick_key_pressed(&self) -> bool { false }
    fn f8_key_pressed(&self) -> bool { false }
    fn f9_key_pressed(&self) -> bool { false }    
    fn flush_input(&mut self) {}
}

#[allow(dead_code)]
pub struct InputHandlerStub;

impl InputHandler for InputHandlerStub {}
