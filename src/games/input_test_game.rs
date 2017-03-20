use super::Game;
use super::GameInput;
use na::{Vector1, Vector2, Vector3, Vector4, Rotation2};
use num::Zero;
use rendering::renderables::Renderable;
use rendering::rectangle::Rectangle;
use rendering::circle::Circle;
use input::keyboard::KeyboardInput;

#[allow(dead_code)]
pub struct InputTestGame {
    rect_pos: Vector2<f64>,
    user_input: Vector2<isize>,
    external_input: InputGameInput
}

impl InputTestGame {
    pub fn new() -> Self {
        InputTestGame {
            rect_pos: Vector2::zero(),
            user_input: Vector2::zero(),
            external_input: InputGameInput::default()
        }
    }
}

impl Game for InputTestGame {
    fn update_input(&mut self) {
        self.user_input.x = self.external_input.kbd.right as isize - (self.external_input.kbd.left as isize);
        self.user_input.y = self.external_input.kbd.up as isize - (self.external_input.kbd.down as isize);
    }

    fn update_logic(&mut self, t_step: f64) {
        self.rect_pos.x = self.rect_pos.x + (self.user_input.x as f64) * t_step;
        self.rect_pos.y = self.rect_pos.y + (self.user_input.y as f64) * t_step;
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let rect = Rectangle {
            length: 0.5,
            height: 0.5,
            rot: Rotation2::new(0.0),
            pos: Vector3::new(self.rect_pos.x, self.rect_pos.y, 0.0),
            color: Vector4::new(0.0, 1.0, 0.0, 1.0)
        };

        vec![Box::new(rect)]
    }

    fn get_input<'a>(&'a mut self) -> Option <&'a mut GameInput> {
        Some(&mut self.external_input)
    }
}

#[derive(Clone, Default)]
pub struct InputGameInput {
    kbd: KeyboardInput,
}

impl GameInput for InputGameInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
