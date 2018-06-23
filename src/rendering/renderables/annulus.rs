use ::rendering::*;
use ::geometry::*;

#[derive(Clone, Debug)]
pub struct Annulus {
    pub radial_dim: Point,
    pub pos: Point3,
    pub color: Color,
    pub fixed: bool
}

impl Annulus {
    pub fn new(radial_dim: Point, pos: Point3, color: Color, fixed: bool) -> Self {
        Self {
            radial_dim,
            pos,
            color,
            fixed
        }
    }

    pub fn new_from_radius_and_thickness(radius: f64, thickness: f64,  pos: Point3, color: Color, fixed: bool) -> Self {
        let radial_dim = Point::new(radius - thickness / 2.0, radius + thickness / 2.0);

        Self {
            radial_dim,
            pos,
            color,
            fixed
        }
    }
}

impl Renderable<StandardPrimitive> for Annulus {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Circ(self.clone().into())] }
}

impl From<Annulus> for CirclePart {
    fn from(ann: Annulus) -> CirclePart {
        CirclePart {
            radial_dim: Point::new(ann.radial_dim.x, ann.radial_dim.y),
            angular_dim: Point::new(0.0, 1.0),
            pos: ann.pos,
            color: ann.color,
            fixed: ann.fixed
        }
    }
}