use rendering::{Rectangle, Circle, Primitive, Renderable};
use na::{Vector2, Vector3, Vector4, Rotation2, norm};

pub struct Line {
    start: Vector2<f64>,
    end: Vector2<f64>,
    thickness: f64,
    shape: LineShape,
    color: Vector4<f64>,
    depth: f64,
    fixed: bool
}

impl Line {
    pub fn new_square(
        start: Vector2<f64>,
        end: Vector2<f64>,
        thickness: f64,
        color: Vector4<f64>,
        depth: f64
    ) -> Self {
        Line {
            start,
            end,
            thickness,
            color,
            depth,
            fixed: false,
            shape: LineShape::Square
        }
    }

    pub fn new_rounded(
        start: Vector2<f64>,
        end: Vector2<f64>,
        thickness: f64,
        color: Vector4<f64>,
        depth: f64
    ) -> Self {
        Line {
            start,
            end,
            thickness,
            color,
            depth,
            fixed: false,
            shape: LineShape::Rounded
        }
    }
}

pub enum LineShape {
    Square,
    Rounded
}

impl Renderable for Line {
    fn get_primitives(&mut self) -> Vec<Primitive> { 
        let shifted_end = self.end - self.start;
        let line_angle = shifted_end.y.atan2(shifted_end.x);
        let midpoint = (self.start + self.end) / 2.0;

        let line_middle = Rectangle {
            length: shifted_end.norm(),
            height: self.thickness,
            pos: Vector3::new(midpoint.x, midpoint.y, self.depth),
            rot: Rotation2::new(line_angle),
            color: self.color,
            fixed: self.fixed
        };

        match self.shape {
            LineShape::Square => return vec![Primitive::Rect(line_middle)],
            LineShape::Rounded => {
                let beg_circ = Circle {
                    radius: self.thickness / 2.0,
                    pos: Vector3::new(self.start.x, self.start.y, self.depth),
                    color: self.color
                };

                let end_circ = Circle {
                    radius: self.thickness / 2.0,
                    pos: Vector3::new(self.end.x, self.end.y, self.depth),
                    color: self.color
                };
                return vec![Primitive::Circ(beg_circ), Primitive::Circ(end_circ), Primitive::Rect(line_middle)]
            }
        }
    }
}