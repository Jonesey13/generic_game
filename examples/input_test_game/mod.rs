use gg::games::Game;
use gg::games::GameInput;
use na::{Vector1, Vector2, Vector3, Vector4, Rotation2};
use num::Zero;
use gg::rendering::renderables::Renderable;
use gg::rendering::rectangle::Rectangle;
use gg::rendering::circle::Circle;
use gg::input::keyboard::KeyboardInput;
use gg::rendering::{BezierRect, BezierQuadControl};
use gg::rendering::{BezierSubrect, BezierLogic};

#[allow(dead_code)]
pub struct InputTestGame {
    obj_pos: Vector2<f64>,
    user_input: Vector2<isize>,
    external_input: InputGameInput
}

impl InputTestGame {
    pub fn new() -> Self {
        InputTestGame {
            obj_pos: Vector2::zero(),
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
        self.obj_pos.x = self.obj_pos.x + (self.user_input.x as f64) * t_step;
        self.obj_pos.y = self.obj_pos.y + (self.user_input.y as f64) * t_step;
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let quad_control = BezierQuadControl {
            one: Vector2::new(0.0, 0.0),
            two: Vector2::new(0.5, 0.2),
            three: Vector2::new(1.0, 0.2),
        };
        let bez_rect = BezierRect::new(quad_control, Vector2::new(0.0, 1.0), 1.0, Vector2::zero(), Vector4::new(0.0, 0.0, 1.0, 1.0));

        let bez_logic = BezierLogic::new(1.0, 2.0, 1.0, 2.0);
        let bez_subrect = BezierSubrect::new(
            bez_rect,
            bez_logic,
            0.1,
            0.1,
            self.obj_pos,
            Vector4::new(0.5, 0.5, 0.5, 0.5)
        );
        
        vec![/*Box::new(rect), Box::new(circ), Box::new(text), */ Box::new(bez_rect), Box::new(bez_subrect)]
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
