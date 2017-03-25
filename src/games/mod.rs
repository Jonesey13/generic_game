pub mod primitive_test_game;
pub mod pong;
pub mod input_test_game;
pub mod physics_test_game;
pub mod view_details;

use rendering::renderables::Renderable;
use input::keyboard::KeyboardInput;
use input::mouse::MouseInput;
use na::{Matrix4, Vector2};
use num::{One, Zero};

/// Game
pub trait Game {
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    #[allow(unused_variables)]
    fn update_logic(&mut self, t_step: f64) {}
    fn get_renderables(&self) -> Vec<Box<Renderable>> { Vec::new()  }
    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> { None }
    fn get_view(&self) -> view_details::ViewDetails {
        view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default())
    }
}

#[allow(dead_code)]
pub struct GameStub;

impl Game for GameStub {}

/// GameInput
pub trait GameInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { None }
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { None }
}

pub struct GameInputStub;

impl GameInput for GameInputStub {}
