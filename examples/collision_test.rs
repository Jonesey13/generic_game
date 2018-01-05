#![feature(set_stdio)]
extern crate generic_game as gg;
extern crate nalgebra as na;
extern crate time;
extern crate num;

use na::{Vector2, Rotation2};
use gg::debug::*;
use gg::debug;
use gg::{rendering, input, window, Handler, games};
use gg::rendering::DisplaySettings;
use gg::collision::{CollisionTestGame, CollisionTestBuilder};
use gg::handler_basic_with_console::HandlerBasicWithConsole;
use gg::geometry::{ConPoly, Circle, Line, Point};
use std::env;
use std::io::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());
    set_panic(Some(error_writer));

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        fullscreen: true,
        text_glyph_detail: 128.0,
            ..Default::default()
    };

    let renderer = rendering::glium_renderer::GliumRenderer::new(display_settings);
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());
    let game: Box<games::Game> = Box::new(
         CollisionTestBuilder::init()
            .add_line(Line::new(Vector2::new(-0.5, -0.2), Vector2::new(-0.5, -0.5)))
            .add_line(Line::new(Vector2::new(-0.8, -0.8), Vector2::new(-0.6, -0.6)))
            .add_circle(Circle::new(0.1, Vector2::new(-0.5, 0.5)))
            .add_circle(Circle::new(0.1, Vector2::new(-0.9, 0.8)))
            .add_point(Point::new(Vector2::new(0.9, -0.9)))
            .add_point(Point::new(Vector2::new(0.7, -0.7)))
            .add_poly(ConPoly::new(vec![
                Vector2::new(0.3, 0.2),
                Vector2::new(0.3, 0.3),
                Vector2::new(0.2, 0.3),
                Vector2::new(0.2, 0.2),                
            ]))
            .add_poly(ConPoly::new(vec![
                Vector2::new(0.8, 0.6),
                Vector2::new(0.8, 0.8),
                Vector2::new(0.7, 0.8),
                Vector2::new(0.7, 0.7),
                Vector2::new(0.6, 0.7),
                Vector2::new(0.6, 0.6)                                                              
            ]))
            .build_game());

    let mut handler: Box<Handler> = Box::new(HandlerBasicWithConsole::new(renderer, input_handler, window_handler, game));

    handler.init();
    while !handler.should_exit() {
        debug_clock_start_main();
        handler.update_input();
        handler.update_rendering();
        handler.update_logic();
        debug_clock_stop_main();
    }
    handler.on_exit();
}
