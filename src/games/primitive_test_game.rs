use super::Game;
use super::GameInput;
use super::view_details::{ViewDetails2D, ViewDetails};
use na::{Vector1, Vector2, Vector3, Vector4, Rotation2, Matrix2, Matrix4};
use num::{Zero, One} ;
use rendering::renderables::Renderable;
use rendering::rectangle::Rectangle;
use rendering::circle::Circle;
use rendering::text::PlainText;
use rendering::{BezierRect, BezierQuadControl};
use input::keyboard::KeyboardInput;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct PrimitiveTestGame {
    view_details: ViewDetails2D,
    user_input: UserInput,
    external_input: ExternalInput
}

impl Game for PrimitiveTestGame {
    fn update_input(&mut self) {
        self.user_input.right_left = self.external_input.kbd.d as isize - (self.external_input.kbd.a as isize);
        self.user_input.up_down = self.external_input.kbd.w as isize - (self.external_input.kbd.s as isize);
        self.user_input.in_out = self.external_input.kbd.r as isize - (self.external_input.kbd.f as isize);
        self.user_input.anticlockwise_clockwise = self.external_input.kbd.e as isize - (self.external_input.kbd.q as isize);
    }

    fn update_logic(&mut self, t_step: f64) {
        self.view_details.camera_pos.x = self.view_details.camera_pos.x + (self.user_input.right_left as f64) * t_step;
        self.view_details.camera_pos.y = self.view_details.camera_pos.y + (self.user_input.up_down as f64) * t_step;
        self.view_details.viewport_height = self.view_details.viewport_height + (self.user_input.in_out as f64) * t_step;
        let current_rotation = self.view_details.get_rotation_angle();
        let new_rotation = current_rotation + (self.user_input.anticlockwise_clockwise as f64) * t_step;
        self.view_details.set_rotation_angle(new_rotation);
    }

    fn get_view(&self) -> ViewDetails {
        ViewDetails::TwoDim(self.view_details.clone())
    }
    
    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let rect = Rectangle {
            length: 0.5,
            height: 0.5,
            rot: Rotation2::new(0.0),
            pos: Vector3::new(0.25, 0.25, 0.1),
            color: Vector4::new(0.0, 1.0, 0.0, 1.0)
        };
        let circ = Circle {
            radius: 0.25,
            pos: Vector3::new(-0.25, -0.25, 0.1),
            color: Vector4::new(1.0, 0.0, 0.0, 1.0)
        };
        let text = PlainText {
            content: "hello there!".to_string(),
            position: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            transform: *Rotation2::new(1.0).matrix(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            fixed: false
        };

        let quad_control = BezierQuadControl {
            one: Vector2::new(0.0, 0.0),
            two: Vector2::new(0.5, 0.2),
            three: Vector2::new(1.0, 0.0),
        };
        let bez_rect = BezierRect::new(quad_control, Vector2::new(1.0, 1.0), 1.0, Vector2::zero(), Vector4::new(0.0, 0.0, 1.0, 1.0));
        
        vec![Box::new(rect), Box::new(circ), Box::new(text), Box::new(bez_rect)]
    }

    fn get_input<'a>(&'a mut self) -> Option <&'a mut GameInput> {
        Some(&mut self.external_input)
    }
}

#[derive(Clone, Default)]
struct UserInput {
    right_left: isize,
    up_down: isize,
    in_out: isize,
    anticlockwise_clockwise: isize
}

#[derive(Clone, Default)]
struct ExternalInput {
    kbd: KeyboardInput,
}

impl GameInput for ExternalInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
