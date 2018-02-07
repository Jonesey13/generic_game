#![feature(set_stdio)]
extern crate generic_game as gg;
extern crate nalgebra as na;
extern crate time;
extern crate num;
extern crate image;
extern crate glium;

use gg::debug::*;
use gg::{debug, rendering, input, window, games, Handler};
use gg::handler_basic_with_console::HandlerBasicWithConsole;
use gg::rendering::DisplaySettings;
use std::env;
use std::io::*;
mod renderable_test_game;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());
    set_panic(Some(error_writer));

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        multisample_level: 8,
        fullscreen: false,
            ..Default::default()
    };

    let image1 = image::load(Cursor::new(&include_bytes!("./renderable_test_game/squirrel.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image1_dimensions = image1.dimensions();
    let image1 = glium::texture::RawImage2d::from_raw_rgba_reversed(&image1.into_raw(), image1_dimensions);
    let texture_array = vec![image1];

    let renderer = rendering::build_renderer_with_textures(display_settings, texture_array);
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());
    let game: Box<games::Game> = Box::new(renderable_test_game::RenderableTestGame::default());
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
