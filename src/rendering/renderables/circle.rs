use rendering::{Renderable, Primitive, CirclePart};
use na::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug)]
pub struct Circle {
    pub radius: f64,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl Circle {
    pub fn new(radius: f64, pos: Vector3<f64>, color: Vector4<f64>, fixed: bool) -> Self {
        Self {
            radius,
            pos,
            color,
            fixed
        }
    }
}

impl Renderable for Circle {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Circ(self.clone().into())] }
}

impl From<Circle> for CirclePart {
    fn from(circ: Circle) -> CirclePart {
        CirclePart {
            radial_dim: Vector2::new(0.0, circ.radius),
            angular_dim: Vector2::new(0.0, 1.0),
            pos: circ.pos,
            color: circ.color,
            fixed: circ.fixed
        }
    }
}