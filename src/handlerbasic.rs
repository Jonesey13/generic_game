use Handler;
use rendering::Renderer;
use input::InputHandler;
use games::Game;
use time;
use debug::*;

pub struct HandlerBasic {
    renderer: Box<Renderer>,
    input_handler: Box<InputHandler>,
    game: Box<Game>,
    last_time: f64,
}

impl HandlerBasic {
    pub fn new(renderer: Box<Renderer>, input_handler: Box<InputHandler>, game: Box<Game>) -> Self {
        HandlerBasic {
            renderer: renderer,
            input_handler: input_handler,
            game: game,
            last_time: 0.0,
        }
    }
}

impl Handler for HandlerBasic {
    fn init(&mut self) {
        self.renderer.init();
        self.input_handler.init();
        self.game.init();
        self.last_time = time::precise_time_s();
    }

    fn update_input(&mut self) {
        debug_clock_start("Input");
        self.input_handler.receive_input();
        self.input_handler.pass_on_input(self.game.get_input());
        self.input_handler.flush_input();
        self.game.update_input();
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
        self.renderer.load_renderables(self.game.get_renderables());
        self.renderer.set_worldview(self.game.get_view());
        self.renderer.render();
        debug_clock_stop("Render");
    }

    fn exit(&self) -> bool {
        self.input_handler.escape_key_pressed()
    }

    fn on_exit(&mut self) {
        self.game.on_exit();
    }
}
