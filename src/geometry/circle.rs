use na::{Vector2, Vector3, Vector4};
use std::fmt;
use geometry::line::Line;
use super::{vect, DualSoln, Poly, TwoDTransformable, ToRenderable};
use rendering;
use collision;

#[derive(Clone)]
pub struct Circle{
    pub rad: f64,
    pub center: Vector2<f64>
}

impl Circle {
    pub fn new(rad: f64, center: Vector2<f64>) -> Circle {
        Circle{
            rad: rad,
            center: center
        }
    }

    pub fn shifted_by(&self, shift: Vector2<f64>) -> Circle {
        let mut out = self.clone();
        out.shift_by(shift);
        out
    }

    pub fn get_movement_line(&self, other: &Circle) -> Line {
        Line::new(self.center, other.center)
    }
}

impl TwoDTransformable for Circle {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        self.center = self.center + shift;
    }

    fn rotate(&mut self, _: f64) {}
}

impl ToRenderable for Circle {
    fn to_renderable(&self, color: Vector4<f64>, depth: f64, _: bool) -> Box<rendering::Renderable> {
        Box::new(rendering::Circle {
            radius: self.rad,
            pos: Vector3::new(self.center.x, self.center.y, depth),
            color
        })
    }
}

impl collision::CollObj for Circle {
    fn get_object_pair(&self, other: &Self) -> collision::CollObjPair {
        collision::CollObjPair::Circ(self.clone(), other.clone())
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle: radius: {}, center: {{ x: {}, y: {} }}", self.rad, self.center.x, self.center.y)
    }
}
