use rendering::{Renderable, Primitive, CirclePart};
use na::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug)]
pub struct Annulus {
    pub radial_dim: Vector2<f64>,
    pub pos: Vector3<f64>,
    pub colour: Vector4<f64>
}

impl Annulus {
    pub fn new(radial_dim: Vector2<f64>, pos: Vector3<f64>, colour: Vector4<f64>) -> Self {
        Self {
            radial_dim,
            pos,
            colour
        }
    }
}

impl Renderable for Annulus {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Circ(self.clone().into())] }
}

impl From<Annulus> for CirclePart {
    fn from(ann: Annulus) -> CirclePart {
        CirclePart {
            radial_dim: Vector2::new(ann.radial_dim.x, ann.radial_dim.y),
            angular_dim: Vector2::new(0.0, 1.0),
            pos: ann.pos,
            colour: ann.colour
        }
    }
}