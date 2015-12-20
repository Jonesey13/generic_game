use Handler;
use rendering::Renderer;
use input::InputHandler;
use games::Game;

pub struct HandlerBasic {
    renderer: Box<Renderer>,
    input_handler: Box<InputHandler>,
    game: Box<Game>,
}

impl HandlerBasic {
    pub fn new(renderer: Box<Renderer>, input_handler: Box<InputHandler>, game: Box<Game>) -> Self {
        HandlerBasic {
            renderer: renderer,
            input_handler: input_handler,
            game: game
        }
    }
}

impl Handler for HandlerBasic {
    fn init(&mut self) {
        self.renderer.init();
        self.input_handler.init();
        self.game.init();
    }

    fn update_input(&mut self) {
        self.input_handler.receive_input();
        self.input_handler.pass_on_input(self.game.get_input());
    }

    fn update_logic(&mut self) {
        self.game.update_logic();
    }

    fn update_rendering(&mut self) {
        self.renderer.load_renderables(self.game.get_renderables());
        self.renderer.render();
    }

    fn exit(&self) -> bool {
        self.input_handler.escape_key_pressed()
    }
}
