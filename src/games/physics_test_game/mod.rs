pub mod coll_circle;
pub mod builder;

use collision::{Collider, Collidable, CollResults, CollDetails};
use na::{Vec2, Vec4};
use self::coll_circle::CollCircle;
use games::Game;
use rendering::renderables::Renderable;
use geometry::FromAngle;
use input::mouse::MouseInput;
use games::GameInput;

pub const RED: Vec4<f64> = Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0};
pub const BLUE: Vec4<f64> = Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0};

pub struct PhysicsTestGame {
    circles: Vec<CollCircle>,
    collider: Collider,
    external_input: PhysicsTestGameInput
}

#[derive(Clone, Default)]
pub struct PhysicsTestGameInput {
    mouse: MouseInput,
}

#[derive(Clone)]
pub enum PhysicsTestObject {
    Circle
}

impl Game for PhysicsTestGame {
    fn update_logic(&mut self, t_step: f64) {
        for circle in &mut self.circles {
            if circle.is_player_controlled()
            {
                circle.set_prev();
                let mouse_inp = self.external_input.mouse.movement;
                let mov_vec = Vec2::new(mouse_inp.0 as f64 * 0.01, - mouse_inp.1 as f64 * 0.01);
                circle.shift_by(mov_vec);
            }
            else {
                circle.update_pos(t_step);
            }
        }

        self.handle_collision();

        for circle in &mut self.circles {
            if circle.has_collided() == true {
                circle.color = RED;
                let coll_dir = match circle.get_collision_details() {
                    Some(CollDetails::Circ(dir)) => dir,
                    _ => panic!("unreachable!")
                };
                let speed = circle.get_speed();
                circle.set_velocity(coll_dir * -speed);
                if let Some(ref prev) = circle.prev.clone() {
                    let collision_time = circle.get_collision_time().unwrap();
                    let next_position = circle.get_pos().clone();
                    circle.set_pos(prev.pos + (next_position - prev.pos) * collision_time);
                }
            }
        }
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let output: Vec<Box<Renderable>> = self.circles.iter().map(|x| {Box::new(x.render()) as Box<Renderable>}).collect();
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
        Some(&mut self.external_input)
    }
}

impl PhysicsTestGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.circles.iter_mut().map(|x| {x as &mut Collidable<Data=PhysicsTestObject>}).collect();
        self.collider.process_all(collidables);
    }
}

impl GameInput for PhysicsTestGameInput {
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { Some(&mut self.mouse) }
}
