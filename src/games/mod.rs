pub mod view_details;

use crate::rendering::Renderable;
use crate::input::keyboard::KeyboardInput;
use crate::input::mouse::MouseInput;
use crate::input::joystick::JoystickInput;
use crate::rendering::{DisplaySettings, WindowSpec};

pub use self::view_details::{ViewDetails, ViewDetails2D, ViewDetails3D};

/// Game
pub trait Game {
    type Primitive;
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    #[allow(unused_variables)]
    fn update_logic(&mut self, t_step: f64) {}
    fn get_renderables(&mut self, _window_spec: WindowSpec) -> Vec<Box<dyn Renderable<Self::Primitive>>> { Vec::new() }
    fn get_input<'a>(&'a mut self) -> Option<&'a mut dyn GameInput> { None }
    fn get_view(&self) -> view_details::ViewDetails {
        view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default())
    }
    fn should_exit(&self) -> bool {false}
    fn on_exit(&mut self) {}
    fn get_console_logs(&mut self) -> Vec<String> { vec![] }
    fn write_to_log(&mut self, _: &str) {}
    fn change_display_settings(&mut self) -> Option<DisplaySettings> {None}
}

/// GameInput
pub trait GameInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { None }
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { None }
    fn get_joystick_inp<'a>(&'a mut self) -> Option<&'a mut JoystickInput> { None }
}

pub struct GameInputStub;

impl GameInput for GameInputStub {}
