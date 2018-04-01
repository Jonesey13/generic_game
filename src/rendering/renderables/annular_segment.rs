use rendering::{Renderable, StandardPrimitive, CirclePart};
use na::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug)]
pub struct AnnularSegment {
    pub radial_dim: Vector2<f64>,
    pub angle_dim: Vector2<f64>,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl AnnularSegment {
    pub fn new(radial_dim: Vector2<f64>, angle_dim: Vector2<f64>, pos: Vector3<f64>, color: Vector4<f64>, fixed: bool) -> Self {
        Self {
            radial_dim,
            angle_dim,
            pos,
            color,
            fixed
        }
    }
}

impl Renderable for AnnularSegment {
    type Primitive = StandardPrimitive;
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Circ(self.clone().into())] }
}

impl From<AnnularSegment> for CirclePart {
    fn from(ann: AnnularSegment) -> CirclePart {
        CirclePart {
            radial_dim: Vector2::new(ann.radial_dim.x, ann.radial_dim.y),
            angular_dim: Vector2::new(ann.angle_dim.x, ann.angle_dim.y),
            pos: ann.pos,
            color: ann.color,
            fixed: ann.fixed
        }
    }
}