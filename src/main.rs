extern crate multiinput;
extern crate nalgebra as na;
extern crate num;
#[macro_use]
extern crate glium;

mod rendering;
mod input;
mod handlerbasic;
mod games;

fn main() {
    let renderer: Box<rendering::Renderer> = Box::new(rendering::RendererStub);
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let game: Box<games::Game> = Box::new(games::GameStub);
    let mut handler: Box<Handler> = Box::new(handlerbasic::HandlerBasic::new(renderer, input_handler, game));

    handler.init();
    while !handler.exit() {
        handler.update_input();
        handler.update_rendering();
        handler.update_logic();
    }
}


/// Handler
pub trait Handler {
    fn exit(&self) -> bool { false }
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self) {}
    fn update_rendering(&mut self) {}
}

#[allow(dead_code)]
pub struct HandlerStub;

impl Handler for HandlerStub{}
