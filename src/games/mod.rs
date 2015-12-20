pub mod rectangle_game;
use rendering::renderables::Renderable;

/// Game
pub trait Game {
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self) {}
    fn get_renderables(&self) -> Vec<Box<Renderable>> { Vec::new()  }
    fn get_input(&self) -> Box<GameInput> { Box::new(GameInputStub) }
}

#[allow(dead_code)]
pub struct GameStub;

impl Game for GameStub {}


/// GameInput
pub trait GameInput {}

pub struct GameInputStub;

impl GameInput for GameInputStub {}
