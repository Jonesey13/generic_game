#![allow(dead_code)]
#![allow(unused_imports)]
extern crate multiinput;
extern crate nalgebra as na;
extern crate num;
#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate time;

mod rendering;
mod input;
mod handlerbasic;
mod games;
mod collision;
mod clock;
mod geometry;

fn main() {
    let renderer: Box<rendering::Renderer> = Box::new(rendering::glium_renderer::GliumRenderer::new((800, 600)));
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    //let game: Box<games::Game> = Box::new(games::pong::builder::PongBuilder::init().build_game());
    let game: Box<games::Game> = Box::new(games::physics_test_game::builder::PhysicsTestBuilder::init().build_game());
    //let game: Box<games::Game> = Box::new(games::input_test_game::InputTestGame::new());
    //let game: Box<games::Game> = Box::new(games::primitive_test_game::PrimitiveTestGame);
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
