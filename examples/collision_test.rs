use generic_game as gg;
use time;

use crate::gg::debug::*;
use crate::gg::debug;
use crate::gg::{rendering, input, window, Handler, games};
use crate::gg::rendering::{DisplaySettings, StandardPrimitive};
use crate::gg::collision::CollisionTestBuilder;
use crate::gg::handler_basic::HandlerBasic;
use crate::gg::geometry::{ConPoly, Circle, Line, Point};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        fullscreen: true,
        text_glyph_detail: 128.0,
            ..Default::default()
    };

    let renderer: Box<dyn rendering::Renderer<Primitive=StandardPrimitive>> = Box::new(rendering::glium_renderer::GliumRenderer::new(display_settings));
    let input_handler: Box<dyn input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<dyn window::WindowHandler> = Box::new(window::GlutinInput::new());
    let game: Box<dyn games::Game<Primitive=StandardPrimitive>> = Box::new(
         CollisionTestBuilder::init()
            .add_line(Line::new(Point::new(-0.5, -0.2), Point::new(-0.5, -0.5)))
            .add_line(Line::new(Point::new(-0.8, -0.8), Point::new(-0.6, -0.6)))
            .add_circle(Circle::new(0.1, Point::new(-0.5, 0.5)))
            .add_circle(Circle::new(0.1, Point::new(-0.9, 0.8)))
            .add_point(Point::new(0.9, -0.9))
            .add_point(Point::new(0.7, -0.7))
            .add_poly(ConPoly::new(vec![
                Point::new(0.3, 0.2),
                Point::new(0.3, 0.3),
                Point::new(0.2, 0.3),
                Point::new(0.2, 0.2),                
            ]))
            .add_poly(ConPoly::new(vec![
                Point::new(0.8, 0.6),
                Point::new(0.8, 0.8),
                Point::new(0.7, 0.8),
                Point::new(0.7, 0.7),
                Point::new(0.6, 0.7),
                Point::new(0.6, 0.6)                                                              
            ]))
            .build_game());

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
