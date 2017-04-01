#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(set_stdio)]
extern crate multiinput;
extern crate nalgebra as na;
extern crate num;
#[macro_use]
extern crate glium;
extern crate time;
#[macro_use]
extern crate bitflags;
extern crate unicode_normalization;
extern crate rusttype;
extern crate rand;

mod rendering;
mod input;
mod handlerbasic;
mod games;
mod collision;
mod clock;
mod geometry;
mod utils;

use na::{Vector2, Vector1, Rotation2};
use utils::debug::*;
use std::env;
use std::io::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    utils::debug::set_flags(DEBUGALL);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());
    set_panic(Some(error_writer));

    let renderer: Box<rendering::Renderer> = Box::new(rendering::glium_renderer::GliumRenderer::new((1600, 1024)));
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    //let game: Box<games::Game> = Box::new(games::pong::builder::PongBuilder::init().build_game());
    //let game: Box<games::Game> = Box::new(
        // games::physics_test_game::builder::PhysicsTestBuilder::init()
        //     .add_rect(Vector2::new(0.5, 0.0), 0.2, 0.2, Rotation2::new(Vector1::new(0.0))).with_velocity(Vector2::new(-0.25, 0.0))
        //     .add_rect(Vector2::new(-0.5, 0.0), 0.2, 0.2, Rotation2::new(Vector1::new(0.0))).with_velocity(Vector2::new(0.25, 0.0))
        //     .build_game());
    //let game: Box<games::Game> = Box::new(games::input_test_game::InputTestGame::new());
    let game: Box<games::Game> = Box::new(games::primitive_test_game::PrimitiveTestGame::default());
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
