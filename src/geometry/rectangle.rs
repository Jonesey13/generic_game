use crate::geometry::*;
use crate::rendering::*;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation,
    pub pos: Point,
}

impl Rectangle {
    pub fn new_regular(
            length: f64, 
            height: f64, 
            pos: Point, 
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation::new(0.0),
            pos,
        }
    }

    pub fn new_corner(
            length: f64, 
            height: f64, 
            corner_pos: Point, 
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation::new(0.0),
            pos: Point::new(corner_pos.x + length / 2.0, corner_pos.y + height / 2.0),
        }
    }

    pub fn new_with_rotation(
            length: f64, 
            height: f64, 
            pos: Point,
            rotation: Rotation,
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: rotation,
            pos,
        }
    }

    pub fn new_corner_with_rotation(
            length: f64, 
            height: f64, 
            corner_pos: Point,
            rotation: Rotation,
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: rotation,
            pos: Point::new(corner_pos.x + length / 2.0, corner_pos.y + height / 2.0),            
        }
    }

    pub fn new_with_whole_rotation_angle(
            length: f64, 
            height: f64, 
            pos: Point,
            angle: f64,
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation::new(2.0 * PI * angle),
            pos,
        }
    }

    pub fn new_regular_fixed(
            length: f64, 
            height: f64, 
            pos: Point, 
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation::new(0.0),
            pos,
        }
    }

    pub fn new_from_line(line: Line, thickness: f64) -> Self {
        let line_length = line.get_length();

        Rectangle {
            length: line_length,
            height: thickness,
            rot: Rotation::new(line.get_angle()),
            pos: line.get_midpoint(),            
        }
    }
}

impl TwoDTransformable for Rectangle {
    fn shift_by(&mut self, shift: Point) {
        self.pos += shift;
    }

    fn rotate_at_center(&mut self, rot_angle: f64) {
        let rot_mat = Rotation::new(rot_angle);
        self.rot = rot_mat * self.rot;
    }

    fn rotate_at_origin(&mut self, rot_angle: f64) {
        let rot_mat = Rotation::new(rot_angle);
        self.rot = rot_mat * self.rot;
        self.pos = rot_mat * self.pos;
    }

    fn get_center(&self) -> Point {
        self.pos
    }

    fn scale_by(&mut self, scale_factor: f64)
    {
        self.length = scale_factor * self.length;
        self.height = scale_factor * self.height;
    }
}

impl ToRenderables for Rectangle {
    fn to_renderables(&self, color: Color, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>> {
        vec![
            Box::new(
                RectanglePrimitive {
                    length: self.length,
                    height: self.height,
                    rot: self.rot,
                    pos: Point3::new(self.pos.x, self.pos.y, depth),
                    color: color,
                    fixed: fixed,
                }
            )
        ]
    }
}

