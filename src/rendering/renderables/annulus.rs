use rendering::{Renderable, Primitive, CirclePart};
use na::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug)]
pub struct Annulus {
    pub radial_dim: Vector2<f64>,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl Annulus {
    pub fn new(radial_dim: Vector2<f64>, pos: Vector3<f64>, color: Vector4<f64>, fixed: bool) -> Self {
        Self {
            radial_dim,
            pos,
            color,
            fixed
        }
    }

    pub fn new_from_radius_and_thickness(radius: f64, thickness: f64,  pos: Vector3<f64>, color: Vector4<f64>, fixed: bool) -> Self {
        let radial_dim = Vector2::new(radius - thickness / 2.0, radius + thickness / 2.0);

        Self {
            radial_dim,
            pos,
            color,
            fixed
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
            color: ann.color,
            fixed: ann.fixed
        }
    }
}