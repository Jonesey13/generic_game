use Handler;
use rendering::{GliumRenderer, Renderer};
use input::InputHandler;
use window::WindowHandler;
use games::Game;
use time;
use debug::*;
use debug::console;
use winapi;

use libloading::{Library, Symbol};

type SetProcessDpiAwareness<'a> = Symbol<'a, unsafe extern "system" fn(awareness: winapi::um::shellscalingapi::PROCESS_DPI_AWARENESS) -> winapi::um::winnt::HRESULT>;

pub struct HandlerBasicWithConsole<'a> {
    renderer: GliumRenderer<'a>,
    input_handler: Box<InputHandler>,
    window_handler: Box<WindowHandler>,
    game: Box<Game>,
    last_time: f64,
    console: console::Console
}

impl<'a> HandlerBasicWithConsole<'a> {
    pub fn new(
        renderer: GliumRenderer<'a>,
        input_handler: Box<InputHandler>,
        window_handler: Box<WindowHandler>,
        game: Box<Game>) -> Self {
        HandlerBasicWithConsole {
            renderer: renderer,
            input_handler: input_handler,
            window_handler: window_handler,
            game: game,
            last_time: 0.0,
            console: console::Console::default()
        }
    }
}

impl<'a> Handler for HandlerBasicWithConsole<'a> {
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
            self.input_handler.pass_on_input(self.game.get_input());
            self.input_handler.flush_input();
            self.game.update_input();
            if self.input_handler.backtick_key_pressed() {
                self.console.toggle()
            }
        }
        debug_clock_stop("Input");
    }

    fn update_logic(&mut self) {
        debug_clock_start("Logic");
        let t_step = time::precise_time_s() - self.last_time;
        self.game.update_logic(t_step);
        self.last_time = self.last_time + t_step;
        debug_clock_stop("Logic");
    }

    fn update_rendering(&mut self) {
        debug_clock_start("Render");
        if let Some(display_settings) = self.game.change_display_settings() {
            self.renderer = GliumRenderer::new(display_settings);
        }
        let window_spec = self.renderer.get_window_spec();
        self.console.write_lines(self.game.get_console_logs());
        self.renderer.load_renderables(self.game.get_renderables());
        self.renderer.load_renderables(self.console.get_renderables(window_spec));
        self.renderer.set_worldview(self.game.get_view());
        self.renderer.render();
        debug_clock_stop("Render");
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