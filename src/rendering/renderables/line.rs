use ::rendering::*;
use na::{Vector3};
use ::geometry::*;

#[derive(Clone, Debug)]
pub struct LineRenderable {
    start: Point,
    end: Point,
    thickness: f64,
    shape: LineShape,
    color: Color,
    depth: f64,
    fixed: bool
}

impl LineRenderable {
    pub fn new(
        start: Point,
        end: Point,
        thickness: f64,
        shape: LineShape,
        color: Color,
        depth: f64,
        fixed: bool
    ) -> Self {
        Self {
            start,
            end,
            thickness,
            shape,
            color,
            depth,
            fixed
        }
    }

    pub fn new_square(
        start: Point,
        end: Point,
        thickness: f64,
        color: Color,
        depth: f64,
        fixed: bool
    ) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
            depth,
            fixed,
            shape: LineShape::Square
        }
    }

    pub fn new_rounded(
        start: Point,
        end: Point,
        thickness: f64,
        color: Color,
        depth: f64,
        fixed: bool
    ) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
            depth,
            fixed,
            shape: LineShape::Rounded
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineShape {
    Square,
    Rounded
}

impl Renderable<StandardPrimitive> for LineRenderable {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { 
        let shifted_end = self.end - self.start;
        let line_angle = shifted_end.y.atan2(shifted_end.x);
        let midpoint = 0.5 * (self.start + self.end);

        let line_middle = RectanglePrimitive {
            length: shifted_end.norm(),
            height: self.thickness,
            pos: Vector3::new(midpoint.x, midpoint.y, self.depth),
            rot: Rotation::new(line_angle),
            color: self.color,
            fixed: self.fixed
        };

        match self.shape {
            LineShape::Square => return vec![StandardPrimitive::Rect(line_middle)],
            LineShape::Rounded => {
                let beg_circ = CircleRenderable {
                    radius: self.thickness / 2.0,
                    pos: Vector3::new(self.start.x, self.start.y, self.depth),
                    color: self.color,
                    fixed: self.fixed
                };

                let end_circ = CircleRenderable {
                    radius: self.thickness / 2.0,
                    pos: Vector3::new(self.end.x, self.end.y, self.depth),
                    color: self.color,
                    fixed: self.fixed
                };
                return vec![StandardPrimitive::Circ(beg_circ.into()), StandardPrimitive::Circ(end_circ.into()), StandardPrimitive::Rect(line_middle)]
            }
        }
    }
}