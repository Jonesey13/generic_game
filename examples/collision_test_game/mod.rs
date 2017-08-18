pub mod coll_circle;
pub mod coll_rect;
pub mod builder;

use gg::collision::{Collider, Collidable};
use na::{Vector2, Rotation2};
use self::coll_circle::CollCircle;
use self::coll_poly::CollPoly;
use gg::games::Game;
use gg::rendering::renderables::Renderable;
use gg::geometry::FromAngle;
use gg::input::mouse::MouseInput;
use gg::input::keyboard::KeyboardInput;
use gg::games::GameInput;

pub const RED: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f64; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct CollisionTestGame {
    circles: Vec<CollCircle>,
    rects: Vec<CollRect>,
    collider: Collider,
    external_input: CollisionTestGameInput,
    mouse_mov: Vector2<f64>,
    mouse_speed: f64
}

#[derive(Clone, Default)]
pub struct CollisionTestGameInput {
    mouse: MouseInput,
    kbd: KeyboardInput,
}

#[derive(Clone)]
pub enum CollisionTestObject {
    Circle,
    Line,
    Point,
    Poly
}

impl Game for CollisionTestGame {
    fn update_logic(&mut self, t_step: f64) {
        self.set_mouse();

        for circle in &mut self.circles {
            circle.set_prev();

            if circle.is_player_controlled()
            {
                circle.shift_by(self.mouse_mov);
            }
            else {
                circle.update_pos(t_step);
            }
        }

        for rect in &mut self.rects {
            rect.set_prev();

            if rect.is_player_controlled()
            {
                rect.shift_by(self.mouse_mov);

                if self.external_input.kbd.left {
                    rect.rotate_by(Rotation2::new(-t_step));
                }
                if self.external_input.kbd.right {
                    rect.rotate_by(Rotation2::new(t_step));
                }
            }
            else {
                rect.update_pos(t_step);
            }
        }

        self.handle_collision();

        for circle in &mut self.circles {
            circle.check_and_resolve_collision();
        }
        for rect in &mut self.rects {
            rect.check_and_resolve_collision();
        }
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let output: Vec<Box<Renderable>> = self.circles.iter().map(|x| {Box::new(x.render()) as Box<Renderable>})
            .chain(self.rects.iter().map(|x| {Box::new(x.render()) as Box<Renderable>})).collect();
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
        Some(&mut self.external_input)
    }
}

impl CollisionTestGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.circles.iter_mut().map(|x| {x as &mut Collidable<Data=PhysicsTestObject>})
            .chain(self.rects.iter_mut().map(|x| {x as &mut Collidable<Data=PhysicsTestObject>})).collect();
        self.collider.process_all(collidables);
    }

    fn set_mouse(&mut self) {
        let ext_mouse_mov = self.external_input.mouse.movement;
        self.mouse_mov = Vector2::new(ext_mouse_mov.0 as f64, -ext_mouse_mov.1 as f64) * self.mouse_speed;
    }
}

impl GameInput for CollisionTestGameInput {
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { Some(&mut self.mouse) }
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
