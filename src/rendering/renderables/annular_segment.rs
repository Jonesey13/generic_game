use ::rendering::*;
use na::{Vector3};
use ::geometry::*;

#[derive(Clone, Debug)]
pub struct AnnularSegment {
    pub radial_dim: Point,
    pub angle_dim: Point,
    pub pos: Vector3<f64>,
    pub color: Color,
    pub fixed: bool
}

impl AnnularSegment {
    pub fn new(radial_dim: Point, angle_dim: Point, pos: Vector3<f64>, color: Color, fixed: bool) -> Self {
        Self {
            radial_dim,
            angle_dim,
            pos,
            color,
            fixed
        }
    }
}

impl Renderable<StandardPrimitive> for AnnularSegment {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Circ(self.clone().into())] }
}

impl From<AnnularSegment> for CirclePart {
    fn from(ann: AnnularSegment) -> CirclePart {
        CirclePart {
            radial_dim: Point::new(ann.radial_dim.x, ann.radial_dim.y),
            angular_dim: Point::new(ann.angle_dim.x, ann.angle_dim.y),
            pos: ann.pos,
            color: ann.color,
            fixed: ann.fixed
        }
    }
}