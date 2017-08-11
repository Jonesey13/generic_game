use na::{Vector2, Vector3, Vector4, norm, Rotation2};
use gg::collision;
use gg::collision::{CollResults, Collidable, CollObj, CollDetails, ConPolyInfo};
use gg::geometry::{circle, con_poly, average_vec2, Poly};
use rendering;
use num::Zero;
use super::RED;
use gg::debug::*;
use gg::debug;

pub struct CollRect {
    pub length: f64,
    pub height: f64,
    pub pos: Vector2<f64>,
    pub color: Vector4<f64>,
    pub velocity: Vector2<f64>,
    pub rot: Rotation2<f64>,
    pub coll_results: CollResults<super::PhysicsTestObject>,
    pub prev: Option<Box<CollRect>>,
    pub player_controlled: bool
}

impl Clone for CollRect {
    fn clone(&self) -> Self {
        CollRect {
            length: self.length,
            height: self.height,
            pos: self.pos.clone(),
            color: self.color.clone(),
            velocity: self.velocity.clone(),
            rot: self.rot.clone(),
            coll_results: self.coll_results.clone(),
            prev: None,
            player_controlled: self.player_controlled,
        }
    }
}

impl CollRect {
    pub fn new(pos: Vector2<f64>, length: f64, height: f64, rot: Rotation2<f64>, color: Vector4<f64>) -> CollRect {
        CollRect {
            length: length,
            height: height,
            pos: pos,
            rot: rot,
            color: color,
            velocity: Vector2::zero(),
            coll_results: CollResults::no_collision(),
            prev: None,
            player_controlled: false,
        }
    }

    pub fn render(&self) -> rendering::rectangle::Rectangle {
        rendering::rectangle::Rectangle  {
            length: self.length,
            height: self.height,
            rot: self.rot,
            pos: Vector3::new(self.pos.x, self.pos.y, 0.0),
            color: self.color
        }
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f64>) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&mut self) -> Vector2<f64> {
        self.velocity
    }

    pub fn set_direction(&mut self, dir: Vector2<f64>) {
        self.velocity = dir.normalize() * self.get_speed();
    }

    pub fn set_pos(&mut self, pos: Vector2<f64>) {
        self.pos = pos;
    }

    pub fn rotate_by(&mut self, rot: Rotation2<f64>) {
        self.rot = rot * self.rot;
    }

    pub fn get_pos(&mut self) -> Vector2<f64> {
        self.pos
    }

    pub fn get_speed(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn get_current_rect(&self) -> con_poly::ConPoly {
        con_poly::ConPoly::new_from_rect(self.length, self.height, self.pos, self.rot)
    }

    pub fn get_previous_rect(&self) -> con_poly::ConPoly {
        if let Some(ref prev_rect) = self.prev {
            return con_poly::ConPoly::new_from_rect(prev_rect.length, prev_rect.height, prev_rect.pos, prev_rect.rot)
        }
        self.get_current_rect()
    }

    pub fn set_speed(&mut self, spd: f64) {
        self.velocity = self.velocity.normalize() * spd;
    }

    pub fn update_pos(&mut self, t_step: f64) {
        self.pos = self.pos + self.velocity * t_step;
    }

    pub fn shift_by(&mut self, mov: Vector2<f64>) {
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

    pub fn check_and_resolve_collision(&mut self) {
        if self.has_collided() {
            self.resolve_collision();
        }
    }

    fn resolve_collision(&mut self) {
        self.color = RED.into();
        let coll_dir = match self.get_collision_details() {
            Some(CollDetails::ConPoly(ConPolyInfo::CornerInfo(_, dir))) => dir,
            Some(CollDetails::ConPoly(ConPolyInfo::LineInfo(index, _))) => self.get_current_rect().get_normal(index),
            Some(CollDetails::ConPoly(ConPolyInfo::SideInfo(index))) => self.get_current_rect().get_normal(index),
            _ => panic!("unreachable!")
        };
        let speed = self.get_speed();

        debug_coll(&format!("Rectangle Collision with coll_details = {:?}", self.get_collision_details()));
        self.set_velocity(coll_dir * -speed);
        if let Some(ref prev) = self.prev.clone() {
            let collision_time = self.get_collision_time().unwrap();
            let next_position = self.get_pos().clone();
            self.set_pos(prev.pos + (next_position - prev.pos) * collision_time);
        }
    }
}

impl Collidable for CollRect {
    type Data = super::PhysicsTestObject;

    fn get_collision_object(&self) -> CollObj {
        CollObj::ConPoly(self.get_current_rect(), self.get_previous_rect())
    }

    fn get_collision_results(&self) -> CollResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_results(&mut self, new_results: CollResults<Self::Data>) {
        self.coll_results = new_results;
    }

    fn get_collision_data(&self) -> Self::Data { super::PhysicsTestObject::Rect }
}
