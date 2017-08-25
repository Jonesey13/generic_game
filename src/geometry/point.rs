use na::{Vector2, Vector3, Vector4};
use std::fmt;
use super::{TwoDTransformable, ToRenderable};
use rendering;
use collision;

#[derive(Clone, Debug)]
pub struct Point {
    pos: Vector2<f64>
}

impl Point {
    pub fn new(pos: Vector2<f64>) -> Point {
        Point {
            pos
        }
    }
}

impl TwoDTransformable for Point {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        self.pos += shift;
    }

    fn rotate(&mut self, _: f64) {}
}

impl ToRenderable for Point {
    fn to_renderable(&self, color: Vector4<f64>, depth: f64, _: bool) -> Box<rendering::Renderable> {
        Box::new(rendering::Circle {
            radius: 0.005,
            pos: Vector3::new(self.pos.x, self.pos.y, depth),
            color
        })
    }
}

impl collision::CollObj for Point {
    fn get_object_pair(&self, other: &Self) -> collision::CollObjPair {
        collision::CollObjPair::Point(self.pos, other.pos)
    }
}