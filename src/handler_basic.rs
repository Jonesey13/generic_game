use Handler;
use rendering::{GliumRenderer, Renderer, StandardPrimitive};
use input::InputHandler;
use window::WindowHandler;
use games::Game;
use time;
use debug::*;
use winapi;
use input::multihandler::MultiInput;

use libloading::{Library, Symbol};

type SetProcessDpiAwareness<'a> = Symbol<'a, unsafe extern "system" fn(awareness: winapi::um::shellscalingapi::PROCESS_DPI_AWARENESS) -> winapi::um::winnt::HRESULT>;

pub struct HandlerBasic<Prim> {
    renderer: Box<Renderer<Primitive=Prim>>,
    input_handler: Box<InputHandler>,
    window_handler: Box<WindowHandler>,
    game: Box<Game<Primitive=Prim>>,
    last_time: f64,
    pause_active_flag: bool,
    increment_frame: bool
}

impl<Prim> HandlerBasic<Prim> {
    pub fn new(
        renderer: Box<Renderer<Primitive=Prim>>,
        input_handler: Box<InputHandler>,
        window_handler: Box<WindowHandler>,
        game: Box<Game<Primitive=Prim>>) -> Self {
        HandlerBasic {
            renderer: renderer,
            input_handler: input_handler,
            window_handler: window_handler,
            game: game,
            last_time: 0.0,
            pause_active_flag: false,
            increment_frame: false
        }
    }

    pub fn pause_active(&self) -> bool {
        self.pause_active_flag && !self.increment_frame
    }
}

impl<Prim> Handler for HandlerBasic<Prim> {
    fn init(&mut self) {
        self.renderer.init();
        self.input_handler.init();
        self.game.init();
        self.last_time = time::precise_time_s();
        set_process_dpi_aware();
    }

    fn update_input(&mut self) {
        debug_clock_start("Input");
        self.window_handler.receive_input(self.renderer.get_events_loop().unwrap());
        
        if self.window_handler.is_focused() {
            self.input_handler.receive_input();

            if self.input_handler.f8_key_pressed() {
                self.pause_active_flag = !self.pause_active_flag;
            }
            self.increment_frame = self.input_handler.f9_key_pressed();

            if !self.pause_active() {
                self.input_handler.pass_on_input(self.game.get_input());
                self.game.update_input();
            }
            self.input_handler.flush_input();
        }
        debug_clock_stop("Input");
    }

    fn update_logic(&mut self) {
        let t_step = time::precise_time_s() - self.last_time;
        if !self.pause_active() {
            debug_clock_start("Logic");
            self.game.update_logic(t_step);
            debug_clock_stop("Logic");
        }
        self.last_time = self.last_time + t_step;
    }

    fn update_rendering(&mut self) {
        if !self.pause_active() {
            debug_clock_start("Render");
            if let Some(display_settings) = self.game.change_display_settings() {
                self.renderer.reset(display_settings);
                self.input_handler.reset();
            }
            let window_spec = self.renderer.get_window_spec();
            self.renderer.load_renderables(self.game.get_renderables(window_spec));
            self.renderer.set_worldview(self.game.get_view());
            self.renderer.render();
            debug_clock_stop("Render");
        }
    }

    fn should_exit(&self) -> bool {
        self.game.should_exit()
    }

    fn on_exit(&mut self) {
        self.game.on_exit();
    }
}

fn set_process_dpi_aware() {
    match Library::new("Shcore.dll") {
        Ok(shcore_lib) => {
            unsafe {
                match shcore_lib.get::<SetProcessDpiAwareness>(b"SetProcessDpiAwareness") {
                    Ok(set_aware) => {
                        set_aware(winapi::um::shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE);
                    },
                    Err(_) => ()
                }
            }
        },
        Err(_) => ()
    };
}