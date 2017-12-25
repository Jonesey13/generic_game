#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(conservative_impl_trait)]

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
#[macro_use]
extern crate lazy_static;
extern crate libloading;
extern crate winapi;

pub mod rendering;
pub mod input;
pub mod handlerbasic;
pub mod handler_basic_with_console;
pub mod games;
pub mod collision;
pub mod geometry;
pub mod utils;
pub mod debug;
pub mod window;

use na::{Vector2, Vector1, Rotation2};
use debug::*;
use std::env;
use std::io::*;


// fn main() {
//     env::set_var("RUST_BACKTRACE", "full");
//     debug::set_flags(DEFAULTDEBUG);
//     debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
//     let error_writer = Box::new(ErrorWriter::new());
//     set_panic(Some(error_writer));

//     let renderer: Box<rendering::Renderer> = Box::new(rendering::glium_renderer::GliumRenderer::new((1600, 1024)));
//     let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
//     let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());
//     //let game: Box<games::Game> = Box::new(games::pong::builder::PongBuilder::init().build_game());
//     //let game: Box<games::Game> = Box::new(
//         // games::physics_test_game::builder::PhysicsTestBuilder::init()
//         //     .add_rect(Vector2::new(0.5, 0.0), 0.2, 0.2, Rotation2::new(Vector1::new(0.0))).with_velocity(Vector2::new(-0.25, 0.0))
//         //     .add_rect(Vector2::new(-0.5, 0.0), 0.2, 0.2, Rotation2::new(Vector1::new(0.0))).with_velocity(Vector2::new(0.25, 0.0))
//         //     .build_game());
//     //let game: Box<games::Game> = Box::new(games::input_test_game::InputTestGame::new());
//     //let game: Box<games::Game> = Box::new(games::primitive_test_game::PrimitiveTestGame::default());
//     let game: Box<games::Game> = Box::new(games::tree_game::TreeGame::new(Default::default()));
    
//     // let game = Box::new(games::polar_game::PolarGameBuilder::default().build_game());
//     let mut handler: Box<Handler> = Box::new(handlerbasic::HandlerBasic::new(renderer, input_handler, window_handler, game));

//     handler.init();
//     while !handler.exit() {
//         debug_clock_start_main();
//         handler.update_input();
//         handler.update_rendering();
//         handler.update_logic();
//         debug_clock_stop_main();
//     }
//     handler.on_exit();
// }

/// Handler
pub trait Handler {
    fn should_exit(&self) -> bool { false }
    fn on_exit(&mut self) {}
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self) {}
    fn update_rendering(&mut self) {}
}

#[allow(dead_code)]
pub struct HandlerStub;

impl Handler for HandlerStub{}
