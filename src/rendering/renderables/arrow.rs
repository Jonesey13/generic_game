use rendering::{Rectangle, Circle, StandardPrimitive, Renderable, Polygon, Line, LineShape};
use na::{Vector3, Vector4};
use geometry::*;
use geometry;

#[derive(Clone, Debug)]
pub struct Arrow {
    pub start: Point,
    pub end: Point,
    thickness: f64,
    arrow_dim: Point,
    color: Vector4<f64>,
    depth: f64,
    fixed: bool,
    shape: ArrowHeadShape
}

#[derive(Clone, Debug)]
pub enum ArrowHeadShape {
    Flat,
    RoundedLine
}

impl Arrow {
    pub fn new(
        start: Point,
        end: Point,
        thickness: f64,
        arrow_dim: Point,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow {
            start,
            end,
            thickness,
            arrow_dim,
            color,
            depth,
            fixed,
            shape: ArrowHeadShape::Flat
        }
    }

    pub fn new_rounded(
        start: Point,
        end: Point,
        thickness: f64,
        arrow_dim: Point,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow {
            start,
            end,
            thickness,
            arrow_dim,
            color,
            depth,
            fixed,
            shape: ArrowHeadShape::RoundedLine
        }
    }

    pub fn new_by_direction(
        start: Point,
        dir: Point,
        length: f64,
        thickness: f64,
        arrow_dim: Point,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow {
            start,
            end: start + length * dir,
            thickness,
            arrow_dim,
            color,
            depth,
            fixed,
            shape: ArrowHeadShape::Flat
        }
    }

    pub fn new_for_coll_test(
        start: Point,
        dir: Point,
        color: Vector4<f64>,
        depth: f64,
        fixed: bool
    ) -> Self {
        Arrow::new_by_direction(
            start,
            dir,
            0.04,
            0.01,
            Point::new(0.02, 0.02),
            color,
            depth,
            fixed
        )
    }

    fn get_center_line(&self) -> geometry::Line {
        geometry::Line::new(self.start, self.end)
    }

    fn get_length(&self) -> f64 {
        let shifted_end = self.end - self.start;
        shifted_end.norm()
    }

    fn get_line_angle(&self) -> f64 {
        let shifted_end = self.end - self.start;
        shifted_end.y.atan2(shifted_end.x)
    }

    fn generate_arrow_head(&self) -> Vec<StandardPrimitive> {
        match self.shape {
            ArrowHeadShape::Flat => self.generate_arrow_head_flat(),
            ArrowHeadShape::RoundedLine => self.generate_arrow_head_rounded()
        }
    }

    fn generate_arrow_head_flat(&self) -> Vec<StandardPrimitive> {
        let arrowhead_points = vec![
            Point::new(0.0, -self.arrow_dim.y), 
            Point::new(self.arrow_dim.x, 0.0),
            Point::new(0.0, self.arrow_dim.y)
        ];
        let arrow_pos = self.get_center_line().get_point(1.0 - self.arrow_dim.x / self.get_length());

        let mut arrowhead = Polygon {
            corners: arrowhead_points,
            center: Point::new(0.0, 0.0),
            rot: Rotation::new(self.get_line_angle()),
            pos: Vector3::new(arrow_pos.x, arrow_pos.y, self.depth),
            color: self.color,
            fixed: self.fixed
        };

        arrowhead.get_primitives()
    }

    pub fn generate_arrow_head_rounded(&self) -> Vec<StandardPrimitive> {
        let arrowhead_points = vec![
            Point::new(0.0, -self.arrow_dim.y), 
            Point::new(self.arrow_dim.x, 0.0),
            Point::new(0.0, self.arrow_dim.y)
        ];
        let arrow_pos = self.get_center_line().get_point(1.0 - self.arrow_dim.x / self.get_length());
        let rotation = Rotation::new(self.get_line_angle());

        let mut left_arrowhead_line = geometry::Line::new(
            rotation * arrowhead_points[0], 
            rotation * arrowhead_points[1]);
        left_arrowhead_line.shift_by(arrow_pos);

        let mut left_line_renderable = Line::new_rounded(
            left_arrowhead_line.beg, 
            left_arrowhead_line.end, 
            self.thickness, 
            self.color, 
            self.depth, 
            self.fixed);

        let mut right_arrowhead_line = geometry::Line::new(
            rotation * arrowhead_points[1], 
            rotation * arrowhead_points[2]);
        right_arrowhead_line.shift_by(arrow_pos);

        let mut right_line_renderable = Line::new_rounded(
            right_arrowhead_line.beg, 
            right_arrowhead_line.end, 
            self.thickness, 
            self.color, 
            self.depth, 
            self.fixed);

        let mut output: Vec<StandardPrimitive> = left_line_renderable.get_primitives();
        output.append(&mut right_line_renderable.get_primitives());
        output
    }
}

impl Renderable<StandardPrimitive> for Arrow {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { 
        let full_length = self.get_length();
        let center_line = geometry::Line::new(self.start, self.end);

        let root_line_length = match self.shape {
            ArrowHeadShape::Flat => full_length - self.arrow_dim.x,
            ArrowHeadShape::RoundedLine => full_length
        };

        let root_line_shape = match self.shape {
            ArrowHeadShape::Flat => LineShape::Square,
            ArrowHeadShape::RoundedLine => LineShape::Rounded
        };

        let mut root_line = Line::new(
            self.start,
            center_line.get_point(root_line_length / full_length),
            self.thickness,
            root_line_shape,
            self.color,
            self.depth,
            self.fixed
        );

        let mut output: Vec<StandardPrimitive> = root_line.get_primitives();
        output.append(&mut self.generate_arrow_head());
        output
    }
}