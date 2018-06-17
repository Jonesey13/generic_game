use na::{Vector3, Vector4, Rotation2};
use geometry::*;
use rendering;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation2<f64>,
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
            rot: Rotation2::new(0.0),
            pos,
        }
    }

    pub fn new_with_rotation(
            length: f64, 
            height: f64, 
            pos: Point,
            rotation: Rotation2<f64>,
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: rotation,
            pos,
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
            rot: Rotation2::new(2.0 * PI * angle),
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
            rot: Rotation2::new(0.0),
            pos,
        }
    }

    pub fn new_from_line(line: Line, thickness: f64) -> Self {
        let line_length = line.get_length();

        Rectangle {
            length: line_length,
            height: thickness,
            rot: Rotation2::new(line.get_angle()),
            pos: line.get_midpoint(),            
        }
    }
}

impl TwoDTransformable for Rectangle {
    fn shift_by(&mut self, shift: Point) {
        self.pos += shift;
    }

    fn rotate(&mut self, rot_angle: f64) {
        let rot_mat = Rotation2::new(rot_angle);
        self.rot = rot_mat * self.rot;
    }
}

impl ToRenderables for Rectangle {
    fn to_renderables(&self, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<rendering::StandardRenderable>> {
        vec![
            Box::new(
                rendering::Rectangle {
                    length: self.length,
                    height: self.height,
                    rot: self.rot,
                    pos: Vector3::new(self.pos.x, self.pos.y, depth),
                    color: color,
                    fixed: fixed,
                }
            )
        ]
    }
}

