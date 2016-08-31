pub mod coll_circle;
pub mod coll_rect;
pub mod builder;

use collision::{Collider, Collidable, CollResults, CollDetails};
use na::{Vec1, Vec2, Vec4, Rot2};
use self::coll_circle::CollCircle;
use self::coll_rect::CollRect;
use games::Game;
use rendering::renderables::Renderable;
use geometry::FromAngle;
use input::mouse::MouseInput;
use input::keyboard::KeyboardInput;
use games::GameInput;

pub const RED: Vec4<f64> = Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0};
pub const BLUE: Vec4<f64> = Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0};

pub struct PhysicsTestGame {
    walls: CollWall,
    generator: Generator,
    collider: Collider,
    external_input: PhysicsTestGameInput,
    mouse_mov: Vec2<f64>,
    mouse_speed: f64
}

#[derive(Clone)]
pub enum PhysicsTestObject {
    Circle,
    Rect,
    Wall,
}

impl Game for PhysicsTestGame {
    fn update_logic(&mut self, t_step: f64) {
        self.set_mouse();

        for circle in &mut self.generator.circles {
            circle.set_prev();
            circle.update_pos(t_step);
        }

        for rect in &mut self.generator.rects {
            rect.set_prev();
            rect.update_pos(t_step);
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

impl PhysicsTestGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.generator.circles.iter_mut().map(|x| {x as &mut Collidable<Data=PhysicsTestObject>})
            .chain(self.generator.rects.iter_mut().map(|x| {x as &mut Collidable<Data=PhysicsTestObject>})).collect();
        self.collider.process_all(collidables);
    }

    fn set_mouse(&mut self) {
        let ext_mouse_mov = self.external_input.mouse.movement;
        self.mouse_mov = Vec2::new(ext_mouse_mov.0 as f64, -ext_mouse_mov.1 as f64) * self.mouse_speed;
    }
}

impl GameInput for PhysicsTestGameInput {
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { Some(&mut self.mouse) }
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
