use Handler;
use rendering::Renderer;
use input::InputHandler;
use games::Game;
use time;
use clock::Clock;

pub struct HandlerBasic {
    renderer: Box<Renderer>,
    input_handler: Box<InputHandler>,
    game: Box<Game>,
    last_time: f64,
    clocks: Clocks,
}

impl HandlerBasic {
    pub fn new(renderer: Box<Renderer>, input_handler: Box<InputHandler>, game: Box<Game>) -> Self {
        HandlerBasic {
            renderer: renderer,
            input_handler: input_handler,
            game: game,
            last_time: 0.0,
            clocks: Clocks::new()
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
        self.clocks.input_clock.start();
        self.input_handler.receive_input();
        self.input_handler.pass_on_input(self.game.get_input());
        self.input_handler.flush_input();
        self.game.update_input();
        self.clocks.input_clock.end();
    }

    fn update_logic(&mut self) {
        self.clocks.logic_clock.start();
        let t_step = time::precise_time_s() - self.last_time;
        self.game.update_logic(t_step);
        self.last_time = self.last_time + t_step;
        self.clocks.logic_clock.end();
    }

    fn update_rendering(&mut self) {
        self.clocks.render_clock.start();
        self.renderer.load_renderables(self.game.get_renderables());
        self.renderer.set_worldview(self.game.get_view());
        self.renderer.render();
        self.clocks.render_clock.end();
    }

    fn exit(&self) -> bool {
        self.input_handler.escape_key_pressed()
    }
}

pub struct Clocks {
    pub input_clock: Clock,
    pub logic_clock: Clock,
    pub render_clock: Clock
}

impl Clocks {
    pub fn new() -> Clocks {
        Clocks {
            input_clock: Clock::new("Input"),
            logic_clock: Clock::new("Logic"),
            render_clock: Clock::new("Render")
        }
    }
}
