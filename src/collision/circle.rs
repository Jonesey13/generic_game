use na::Vec2;
use super::Collidable;
use super::CollObj;

pub struct Circle {
    center: Vec2<f64>,
    radius: f64,
    collided: bool,
    collision_point: Option<Vec2<f64>>
}

impl Circle {
    pub fn new (center: Vec2<f64>, radius: f64) -> Circle {
        Circle {
            center: center,
            radius: radius,
            collided: false,
            collision_point: None
        }
    }
}
