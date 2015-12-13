use rendering::renderables::{Renderables, RenderablesStub};

/// Game
pub trait Game {
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self) {}
    fn get_renderables(&self) -> Box<Renderables> { Box::new(RenderablesStub) }
    fn get_input(&self) -> Box<GameInput> { Box::new(GameInputStub) }
}

pub struct GameStub;

impl Game for GameStub {}


/// GameInput
pub trait GameInput {}

pub struct GameInputStub;

impl GameInput for GameInputStub {}
