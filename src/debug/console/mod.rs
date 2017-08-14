mod console_stack;

use rendering::{Renderable, Rectangle, PlainText, TextAlign};
use self::console_stack::ConsoleStack;
use na::{Vector2, Vector3, Vector4};

const CONSOLE_PADDING: f64 = 0.001;

pub struct Console {
    config: RenderConfig,
    stack: ConsoleStack,
    active: bool
}

impl Default for Console {
    fn default() -> Self {
        let config = RenderConfig {
            line_size: 0.05,
            console_position: ConsolePos::Bottom,
            console_size: 0.5
        };

        Console::new(config)
    } 
}

impl Console {
    pub fn new(config: RenderConfig) -> Console {
        Console {
            config,
            stack: ConsoleStack::new(),
            active: false
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.stack.push("Starting Log");
        self.active = true;
    }

    pub fn toggle(&mut self) {
        match self.active {
            true => self.deactivate(),
            false => self.activate()
        }
    }

    pub fn write(&mut self, console_log: &str) {
        self.stack.push(console_log);
    }

    pub fn write_lines(&mut self, log: Vec<String>) {
        for string in log {
            self.write(&string);
        }
    }

    pub fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let mut output: Vec<Box<Renderable>> = Vec::new();
        if self.active {
            output.push(self.get_console_frame_renderable());

            output.append(&mut self.get_log_line_renderables());
        }
        output
    }

    fn get_console_frame_renderable(&self) -> Box<Renderable> {
        let (length, height) = match self.config.console_position {
            ConsolePos::Bottom | ConsolePos::Top => (2.0, self.config.console_size),
            ConsolePos::Left | ConsolePos::Right => (self.config.console_size, 2.0)
        };

        let (posx, posy) = match self.config.console_position {
            ConsolePos::Bottom => (0.0, - (2.0 - self.config.console_size) / 2.0),
            ConsolePos::Top => (0.0, (2.0 - self.config.console_size) / 2.0),
            ConsolePos::Left => (- (2.0 - self.config.console_size) / 2.0, 0.0),
            ConsolePos::Right => ((2.0 - self.config.console_size) / 2.0, 0.0)                                    
        };

        let pos = Vector3::new(posx, posy, -1.0);
        let color = 0.2 * Vector4::new(1.0, 1.0, 1.0, 1.0);

        Box::new(Rectangle::new_regular_fixed(length, height, pos, color))
    }

    fn get_log_line_renderables(&self) -> Vec<Box<Renderable>> {
        let mut output: Vec<Box<Renderable>> = Vec::new();

        for (index, text) in self.stack.get_recent_entries(self.config.get_total_lines()).enumerate() {
            let pos = self.config.get_root_position() - index as f64 * self.config.line_size * Vector2::y();
            let plain_text = PlainText::new_simple_white(text.clone(), self.config.line_size, Vector3::new(pos.x, pos.y, -1.0), TextAlign::Left);
            output.push(Box::new(plain_text));
        }

        output
    }
}

pub struct RenderConfig {
    line_size: f64,
    console_position: ConsolePos,
    console_size: f64
}

impl RenderConfig {
    pub fn get_total_lines(&self) -> usize {
        (self.console_size / self.line_size).floor() as usize 
    }

    pub fn get_root_position(&self) -> Vector2<f64> {
        match self.console_position {
            ConsolePos::Top | ConsolePos::Left => Vector2::new(-1.0 + CONSOLE_PADDING, 1.0 - self.line_size / 2.0 - CONSOLE_PADDING),
            ConsolePos::Bottom => Vector2::new(-1.0 + CONSOLE_PADDING, - 1.0 + self.console_size - self.line_size / 2.0 - CONSOLE_PADDING),
            ConsolePos::Right => Vector2::new(- 1.0 + self.console_size + CONSOLE_PADDING, 1.0 - self.line_size / 2.0 - CONSOLE_PADDING),
        }
    }
}

pub enum ConsolePos {
    Left,
    Right,
    Top,
    Bottom
}