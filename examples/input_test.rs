#![feature(set_stdio)]
extern crate generic_game as gg;
extern crate nalgebra as na;
extern crate time;
extern crate num;

use na::{Vector2, Vector1, Rotation2};
use gg::debug::*;
use gg::debug;
use gg::{rendering, input, window, handlerbasic, Handler, games};
use std::env;
use std::io::*;

mod input_test_game;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());
    set_panic(Some(error_writer));

    let renderer: Box<rendering::Renderer> = Box::new(rendering::glium_renderer::GliumRenderer::new((1600, 1024)));
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());
    let game: Box<games::Game> = Box::new(input_test_game::InputTestGame::new());
    
    let mut handler: Box<Handler> = Box::new(handlerbasic::HandlerBasic::new(renderer, input_handler, window_handler, game));

    handler.init();
    while !handler.exit() {
        debug_clock_start_main();
        handler.update_input();
        handler.update_rendering();
        handler.update_logic();
        debug_clock_stop_main();
    }
    handler.on_exit();
}