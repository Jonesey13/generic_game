use generic_game as gg;
use time;
use image;


use crate::gg::debug::*;
use crate::gg::{debug, rendering, input, window, games, Handler};
use crate::gg::handler_basic::HandlerBasic;
use crate::gg::rendering::{DisplaySettings, StandardPrimitive};
use std::env;
use std::io::*;
mod renderable_test_game;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        multisample_level: 8,
        fullscreen: false,
            ..Default::default()
    };

    let image1 = image::load(Cursor::new(&include_bytes!("./renderable_test_game/Racing2.png")[..]),
                        image::PNG).unwrap();
    let texture_array = vec![image1];

    let renderer = Box::new(rendering::GliumRenderer::new_with_textures(display_settings, texture_array));
    let input_handler: Box<dyn input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<dyn window::WindowHandler> = Box::new(window::GlutinInput::new());
    let game: Box<dyn games::Game<Primitive=StandardPrimitive>> = Box::new(renderable_test_game::RenderableTestGame::default());
    let mut handler: Box<dyn Handler> = Box::new(HandlerBasic::new(renderer, input_handler, window_handler, game));

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
