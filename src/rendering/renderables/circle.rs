use crate::rendering::*;
use crate::geometry::*;

#[derive(Clone, Debug)]
pub struct CircleRenderable {
    pub radius: f64,
    pub pos: Point3,
    pub color: Color,
    pub fixed: bool
}

impl CircleRenderable {
    pub fn new(radius: f64, pos: Point3, color: Color, fixed: bool) -> Self {
        Self {
            radius,
            pos,
            color,
            fixed
        }
    }
}

impl Renderable<StandardPrimitive> for CircleRenderable {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Circ(self.clone().into())] }
}

impl From<CircleRenderable> for CirclePart {
    fn from(circ: CircleRenderable) -> CirclePart {
        CirclePart {
            radial_dim: Point::new(0.0, circ.radius),
            angular_dim: Point::new(0.0, 1.0),
            pos: circ.pos,
            color: circ.color,
            fixed: circ.fixed
        }
    }
}