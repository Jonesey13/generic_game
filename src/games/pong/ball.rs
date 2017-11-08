use na::{Vector2, Vector3, Vector4, norm};
use num::Zero;
use geometry::circle;
use collision::{Collidable, CollisionObject, CollisionObjectResults};
use rendering;
use super::FOREGROUND_LAYER;

pub struct Ball {
    position: Vector2<f64>,
    radius: f64,
    color: Vector4<f64>,
    velocity: Vector2<f64>,
    coll_results: CollisionObjectResults<super::PongObject>,
    prev: Option<Box<Ball>>,
}

impl Clone for Ball {
    fn clone(&self) -> Self {
        Ball {
            position: self.position.clone(),
            radius: self.radius,
            color: self.color.clone(),
            velocity: self.velocity.clone(),
            coll_results: self.coll_results.clone(),
            prev: None
        }
    }
}

impl Ball {
    pub fn new(pos: Vector2<f64>, rad: f64, color: Vector4<f64>) -> Ball {
        Ball {
            position: pos,
            radius: rad,
            color: color,
            velocity: Vector2::zero(),
            coll_results: CollisionObjectResults::no_collision(),
            prev: None
        }
    }

    pub fn render(&self) -> rendering::circle::Circle {
        rendering::circle::Circle {
            radius: self.radius,
            pos: Vector3::new(self.position.x, self.position.y, FOREGROUND_LAYER),
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

    pub fn get_speed(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn get_current_circle(&self) -> circle::Circle {
        circle::Circle::new(self.radius, self.position)
    }

    pub fn get_previous_circle(&self) -> circle::Circle {
        if let Some(ref prev_circ) = self.prev {
            return circle::Circle::new(prev_circ.radius, prev_circ.position);
        }
        self.get_current_circle()
    }

    pub fn set_speed(&mut self, spd: f64) {
        self.velocity = self.velocity.normalize() * spd;
    }

    pub fn update_pos(&mut self, t_step: f64) {
        self.position = self.position + self.velocity * t_step;
    }
}

impl Collidable for Ball {
    type Data = super::PongObject;

    fn get_collision_object(&self) -> CollisionObject {
        CollisionObject::Circ(self.get_current_circle(), self.get_previous_circle())
    }

    fn get_collision_object_results(&self) -> CollisionObjectResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_object_results(&mut self, new_results: CollisionObjectResults<Self::Data>) {
        self.coll_results = new_results;
    }

    fn get_collision_data(&self) -> Self::Data { super::PongObject::Ball }
}
