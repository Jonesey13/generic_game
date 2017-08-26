use rendering::{Rectangle, Circle, Primitive, Renderable, Polygon};
use na::{Vector2, Vector3, Vector4, Rotation2, norm};
use geometry::ConPoly;
use geometry;

pub struct Arrow {
    start: Vector2<f64>,
    end: Vector2<f64>,
    thickness: f64,
    arrow_size: f64,
    color: Vector4<f64>,
    depth: f64,
    fixed: bool
}

impl Arrow {
    pub fn new(
        start: Vector2<f64>,
        end: Vector2<f64>,
        thickness: f64,
        arrow_size: f64,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow {
            start,
            end,
            thickness,
            arrow_size,
            color,
            depth,
            fixed
        }
    }

    pub fn new_by_direction(
        start: Vector2<f64>,
        dir: Vector2<f64>,
        length: f64,
        thickness: f64,
        arrow_size: f64,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow {
            start,
            end: start + length * dir,
            thickness,
            arrow_size,
            color,
            depth,
            fixed
        }
    }

    pub fn new_for_coll_test(
        start: Vector2<f64>,
        dir: Vector2<f64>,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow::new_by_direction(
            start,
            dir,
            0.04,
            0.01,
            0.3,
            color,
            depth,
            fixed
        )
    }
}

impl Renderable for Arrow {
    fn get_primitives(&mut self) -> Vec<Primitive> { 
        let shifted_end = self.end - self.start;
        let length = shifted_end.norm();
        let line_angle = shifted_end.y.atan2(shifted_end.x);
        let center_line = geometry::Line::new(self.start, self.end);
        let midpoint = (self.start + center_line.get_point(1.0 - self.arrow_size)) / 2.0;

        let mut root_line = Rectangle {
            length: length * (1.0 - self.arrow_size),
            height: self.thickness,
            pos: Vector3::new(midpoint.x, midpoint.y, self.depth),
            rot: Rotation2::new(line_angle),
            color: self.color,
            fixed: self.fixed
        };

        let arrowhead_points = vec![
            Vector2::new(0.0, -self.thickness), 
            Vector2::new(length * self.arrow_size, 0.0),
            Vector2::new(0.0, self.thickness)
        ];
        let arrow_pos = center_line.get_point(1.0 - self.arrow_size);

        let mut arrowhead = Polygon {
            corners: arrowhead_points,
            center: Vector2::new(0.0, 0.0),
            rot: Rotation2::new(line_angle),
            pos: Vector3::new(arrow_pos.x, arrow_pos.y, self.depth),
            color: self.color,
            fixed: self.fixed
        };

        let mut output = root_line.get_primitives();
        output.append(&mut arrowhead.get_primitives());
        output
    }
}