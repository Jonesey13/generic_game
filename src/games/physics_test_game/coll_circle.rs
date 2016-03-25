use na::{Vec2, Vec3, Vec4, Norm};
use collision;
use collision::{CollResults, Collidable, CollObj};
use geometry::circle;
use rendering;
use num::Zero;

pub struct CollCircle {
    pub rad: f64,
    pub pos: Vec2<f64>,
    pub color: Vec4<f64>,
    pub velocity: Vec2<f64>,
    pub coll_results: CollResults<super::PhysicsTestObject>,
    pub prev: Option<Box<CollCircle>>,
    pub player_controlled: bool
}

impl Clone for CollCircle {
    fn clone(&self) -> Self {
        CollCircle {
            pos: self.pos.clone(),
            rad: self.rad,
            color: self.color.clone(),
            velocity: self.velocity.clone(),
            coll_results: self.coll_results.clone(),
            prev: None,
            player_controlled: self.player_controlled,
        }
    }
}

impl CollCircle {
    pub fn new(pos: Vec2<f64>, rad: f64, color: Vec4<f64>) -> CollCircle {
        CollCircle {
            pos: pos,
            rad: rad,
            color: color,
            velocity: Vec2::zero(),
            coll_results: CollResults::no_collision(),
            prev: None,
            player_controlled: false,
        }
    }

    pub fn render(&self) -> rendering::circle::Circle {
        rendering::circle::Circle {
            radius: self.rad,
            pos: Vec3::new(self.pos.x, self.pos.y, 0.0),
            color: self.color
        }
    }

    pub fn set_velocity(&mut self, velocity: Vec2<f64>) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&mut self) -> Vec2<f64> {
        self.velocity
    }

    pub fn set_direction(&mut self, dir: Vec2<f64>) {
        self.velocity = dir.normalize() * self.get_speed();
    }

    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos;
    }

    pub fn get_pos(&mut self) -> Vec2<f64> {
        self.pos
    }

    pub fn get_speed(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn get_current_circle(&self) -> circle::Circle {
        circle::Circle::new(self.rad, self.pos)
    }

    pub fn get_previous_circle(&self) -> circle::Circle {
        if let Some(ref prev_circ) = self.prev {
            return circle::Circle::new(prev_circ.rad, prev_circ.pos);
        }
        self.get_current_circle()
    }

    pub fn set_speed(&mut self, spd: f64) {
        self.velocity = self.velocity.normalize() * spd;
    }

    pub fn update_pos(&mut self, t_step: f64) {
        self.prev = Some(Box::new(self.clone()));
        self.pos = self.pos + self.velocity * t_step;
    }

    pub fn shift_by(&mut self, mov: Vec2<f64>) {
        self.pos = self.pos + mov;
    }

    pub fn set_prev(&mut self) {
        self.prev = Some(Box::new(self.clone()));
    }

    pub fn is_player_controlled(&self) -> bool {
        self.player_controlled
    }

    pub fn toggle_player_control(&mut self) {
        self.player_controlled = !self.player_controlled;
    }
}

impl Collidable for CollCircle {
    type Data = super::PhysicsTestObject;

    fn get_collision_object(&self) -> CollObj {
        CollObj::Circ(self.get_current_circle(), self.get_previous_circle())
    }

    fn get_collision_results(&self) -> CollResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_results(&mut self, new_results: CollResults<Self::Data>) {
        self.coll_results = new_results;
    }

    fn get_collision_data(&self) -> Self::Data { super::PhysicsTestObject::Circle }
}
